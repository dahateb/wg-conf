use registration::registration_server::{Registration, RegistrationServer};
use registration::{RegisterReply, RegisterRequest};
use tonic::{transport::Server, Request, Response, Status};
use backend::WireguardConfig;

pub mod registration {
    tonic::include_proto!("registration");
}

pub mod backend;

const CONF_FILE_NAME: &str = "test.ini";

pub struct WgRegistration {
    config: WireguardConfig
}

impl WgRegistration {

    pub fn new() -> WgRegistration {
        WgRegistration {
            config: WireguardConfig::new(CONF_FILE_NAME)
        }
    }

}

#[tonic::async_trait]
impl Registration for WgRegistration {
    async fn register_client(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterReply>, Status> {
        println!("Got a request: {:?}", request);

        let client_public_key = request.into_inner().public_key;
        let reply = registration::RegisterReply {
            public_key: self.config.get_public_key(),
            ipv4_address: format!("{}", self.config.get_ipv4(client_public_key).unwrap()),
            ipv6_address: format!("{}", self.config.get_ipv6().unwrap()),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
pub async fn start_server(ip: &str, port: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("starting server on port {}", port);
    let addr = format!("{}:{}", ip, port).parse().unwrap();
    let registration = WgRegistration::new();
    Server::builder()
        .add_service(RegistrationServer::new(registration))
        .serve(addr)
        .await?;
    Ok(())
}
