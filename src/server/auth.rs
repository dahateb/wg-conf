use tonic::{
    Request, Status,
};

pub fn intercept(req: Request<()>) -> Result<Request<()>, Status> {
    info!("Intercepting request: {:?}", req);
    Ok(req)
}