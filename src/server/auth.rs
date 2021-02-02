use tonic::{Request, Status};

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
