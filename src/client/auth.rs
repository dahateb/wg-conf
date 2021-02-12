use tonic::{Request, Status};

pub fn interceptor(
    script_name: &str,
) -> Box<dyn Fn(Request<()>) -> Result<Request<()>, Status> + Send + Sync + 'static> {
    //script auth currently not possible as interceptor is not async :(
    let script_name_static: &'static str = Box::leak(script_name.into());
    let intercept = move |req: Request<()>| {
        info!("Calling {}, Intercepting request: {:?}", script_name_static, req);
        Ok(req)
    };
    return Box::new(intercept);
}
