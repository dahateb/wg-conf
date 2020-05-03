use registration::registration_server::{Registration, RegistrationServer};
use registration::{RegisterReply, RegisterRequest};
use tonic::{transport::Server, Request, Response, Status};

pub mod registration {
    tonic::include_proto!("registration");
}

#[derive(Debug, Default)]
pub struct WgRegistration {}

#[tonic::async_trait]
impl Registration for WgRegistration {
    async fn register_client(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = registration::RegisterReply {
            public_key: "123456".into(),
            ip_address: "23456".into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
pub async fn start_server(ip: &str, port: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("starting server on port {}", port);
    let addr = format!("{}:{}", ip, port).parse().unwrap();
    let registration = WgRegistration::default();
    Server::builder()
        .add_service(RegistrationServer::new(registration))
        .serve(addr)
        .await?;
    Ok(())
}
