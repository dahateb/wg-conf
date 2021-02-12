use http::Request as HyperRequest;
use hyper::{Body, Response as HyperResponse};
use std::task::{Context, Poll};
use tonic::{body::BoxBody, transport::NamedService, Request, Status};
use tower::Service;

pub fn interceptor(
    script_name: &'static str,
) -> Box<dyn Fn(Request<()>) -> Result<Request<()>, Status> + Send + Sync + 'static> {
    //script auth currently not possible as interceptor is not async :(
    let intercept = move |req: Request<()>| {
        info!("Calling {}, Intercepting request: {:?}", script_name, req);
        Ok(req)
    };
    return Box::new(intercept);
}

async fn auth_check(auth_file_name: String) -> bool {
    return true;
}

// instead of interceptor to handle async function
#[derive(Debug, Clone)]
pub struct InterceptedService<S> {
    inner: S,
    auth_script_file:String
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
        Box::pin(async move {
            // Do async work here....
            if auth_check(auth_file_script).await {
                return Ok(http::Response::builder()
                    .status("401")
                    .body(tonic::body::BoxBody::empty())
                    .unwrap());
            }
            svc.call(req).await
        })
    }
}

impl<S: NamedService> NamedService for InterceptedService<S> {
    const NAME: &'static str = S::NAME;
}
