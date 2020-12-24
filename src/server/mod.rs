use backend::WireguardConfig;
use registration::registration_server::{Registration, RegistrationServer};
use registration::{RegisterReply, RegisterRequest};
use tonic::{transport::Server, Request, Response, Status};

pub mod registration {
    tonic::include_proto!("registration");
}

pub mod backend;

pub struct WgRegistration {
    config: WireguardConfig,
    wg_port: String,
}

impl WgRegistration {
    pub fn new(wg_port: &str, config_file: &str) -> WgRegistration {
        WgRegistration {
            config: WireguardConfig::new(config_file),
            wg_port: wg_port.into(),
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
        let registration = self
            .config
            .register(client_public_key)
            .map_err(|e| error!("{}", e))
            .unwrap();
        let ipv6 = match registration.ipv6_addr {
            Some(addr) => format!("{}", addr),
            None => String::new(),
        };
        let reply = registration::RegisterReply {
            public_key: registration.public_key,
            ipv4_address: format!("{}", registration.ipv4_addr),
            ipv6_address: ipv6,
            wg_port: self.wg_port.clone(),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
pub async fn start_server(
    ip: &str,
    port: &str,
    wg_port: &str,
    config_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("starting server on port {}", port);
    let addr = format!("{}:{}", ip, port).parse().unwrap();
    let registration = WgRegistration::new(wg_port, config_file);
    Server::builder()
        .add_service(RegistrationServer::new(registration))
        .serve(addr)
        .await?;
    Ok(())
}
