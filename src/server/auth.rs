use hooks::run;
use http::{HeaderValue, Request as HyperRequest};
use hyper::{Body, Response as HyperResponse};
use std::task::{Context, Poll};
use tonic::{body::BoxBody, transport::NamedService, Status};
use tower::Service;

async fn auth_check(auth_file_name: String, auth_header: HeaderValue) -> bool {
    let auth_header = auth_header.to_str().unwrap();
    info!("Auth Header: {}", auth_header);
    let result = run(&format!("{} {}", auth_file_name, auth_header)).await;
    if result.is_ok() {
        info!("{}", result.unwrap());
        return true;
    }
    error!("{}", result.unwrap_err());
    return false;
}

// instead of interceptor to handle async function
#[derive(Debug, Clone)]
pub struct InterceptedService<S> {
    inner: S,
    auth_script_file: String,
}

impl<S> InterceptedService<S> {
    pub fn new(inner: S, auth_file_name: String) -> InterceptedService<S> {
        InterceptedService {
            inner: inner,
            auth_script_file: auth_file_name,
        }
    }
}

impl<S> Service<HyperRequest<Body>> for InterceptedService<S>
where
    S: Service<HyperRequest<Body>, Response = HyperResponse<BoxBody>>
        + NamedService
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: HyperRequest<Body>) -> Self::Future {
        let mut svc = self.inner.clone();
        let auth_file_script = self.auth_script_file.clone();
        let auth_header = req.headers().get("authorization").map(ToOwned::to_owned);
        Box::pin(async move {
            // Do async work here....
            if auth_header.is_none() || !auth_check(auth_file_script, auth_header.unwrap()).await {
                return Ok(Status::unauthenticated("unauthorized").to_http());
            }
            svc.call(req).await
        })
    }
}

impl<S: NamedService> NamedService for InterceptedService<S> {
    const NAME: &'static str = S::NAME;
}
