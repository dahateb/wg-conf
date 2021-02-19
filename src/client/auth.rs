
use base64::{encode};
use tonic::{
    metadata::{Ascii, MetadataValue},
    Request, Status,
};

pub fn interceptor(
    auth: AuthBuilder,
) -> Box<dyn Fn(Request<()>) -> Result<Request<()>, Status> + Send + Sync + 'static> {
    let auth_header_value: &'static str = Box::leak(auth.get_auth().unwrap().into_boxed_str());
    let intercept = move |mut req: Request<()>| {
        let token: MetadataValue<Ascii> = MetadataValue::from_str(auth_header_value).unwrap();
        info!(
            "adding auth header {}, Intercepting request: {:?}",
            auth_header_value, req
        );
        req.metadata_mut().insert("authorization", token);
        Ok(req)
    };
    return Box::new(intercept);
}

pub struct AuthBuilder {
    auth_token: Option<String>,
    auth_username: Option<String>,
    auth_password: Option<String>,
}

impl AuthBuilder {
    pub fn new(
        auth_token: Option<&str>,
        auth_username: Option<&str>,
        auth_password: Option<&str>,
    ) -> AuthBuilder {
        AuthBuilder {
            auth_token: auth_token.map(|s| s.to_string()),
            auth_username: auth_username.map(|s| s.to_string()),
            auth_password: auth_password.map(|s| s.to_string()),
        }
    }

    pub fn has_authentication(&self) -> bool {
        self.auth_token.is_some() || self.auth_username.is_some() && self.auth_password.is_some()
    }

    pub fn get_auth(&self) -> Result<String, String> {
        if self.auth_token.is_some() {
            return Ok("Bearer ".to_owned() + self.auth_token.as_ref().unwrap());
        }
        if self.auth_username.is_some() && self.auth_password.is_some() {
            let mut auth = self.auth_username.as_ref().unwrap().clone();
            auth.push_str(":");
            auth.push_str(self.auth_password.as_ref().unwrap());
            return Ok("Basic ".to_owned() + &encode(auth));
        }
        return Err("no authentication defined".to_string());
    }
}
