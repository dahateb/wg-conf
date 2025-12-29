use common::AuthType;
use hooks::run;
use std::io::Write;
use std::str::FromStr;
use std::sync::Arc;
use tempfile::NamedTempFile;
use tonic::codegen::http::Request;
use tonic::{Status, body::Body};
use tonic_middleware::RequestInterceptor;

use tonic::async_trait;

#[async_trait]
pub trait AuthService: Send + Sync + 'static {
    async fn verify_token(&self, token: &str) -> Result<String, String>;
}

#[derive(Default, Clone)]
pub struct AuthServiceImpl {
    auth_script_file: String,
}

impl AuthServiceImpl {
    pub fn new(auth_script_file: String) -> AuthServiceImpl {
        AuthServiceImpl { auth_script_file }
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn verify_token(&self, token: &str) -> Result<String, String> {
        let auth_file_script = self.auth_script_file.clone();
        let header_parts: Vec<&str> = token.split_whitespace().collect();
        let mut tmp_file = NamedTempFile::new().unwrap();
        // check auth type
        let auth_type = AuthType::from_str(header_parts[0]);
        if auth_type.is_err() {
            error!(
                "Got wrong or unsupported auth type: {} - {}",
                header_parts[0],
                auth_type.unwrap_err()
            );
            return Err("".to_string());
        }
        /*  write credentials in tempfile  */
        let _ = writeln!(tmp_file, "{}", header_parts[1]).is_ok();
        info!("Auth Header: {} {}", header_parts[0], header_parts[1]);
        let tmp_file_path = tmp_file.path();
        let result = run(&format!(
            "{} '{}' '{}'",
            auth_file_script,
            auth_type.unwrap().to_string(),
            tmp_file_path.to_str().unwrap()
        ))
        .await;
        let _ = tmp_file.close();
        if result.is_ok() {
            info!("Result: {}", result.unwrap());
            return Ok("".to_string());
        }
        error!("{}", result.unwrap_err());
        Err("".to_string())
    }
}

#[derive(Clone)]
pub struct AuthInterceptor<A: AuthService> {
    pub auth_service: Arc<A>,
}

#[async_trait]
impl<A: AuthService> RequestInterceptor for AuthInterceptor<A> {
    async fn intercept(&self, req: Request<Body>) -> Result<Request<Body>, Status> {
        match req.headers().get("authorization").map(|v| v.to_str()) {
            Some(Ok(token)) => {
                let res = self.auth_service.verify_token(token).await;
                if res.is_ok() {
                    return Ok(req);
                }
                Err(Status::unauthenticated("Unauthenticated"))
            }
            _ => Err(Status::unauthenticated("Unauthenticated")),
        }
    }
}
