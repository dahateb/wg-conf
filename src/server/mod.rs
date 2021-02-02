use auth::interceptor;
use config::WireguardConfig;
use hooks::RegistrationHooks;
use registration::registration_server::{Registration, RegistrationServer};
use registration::{RegisterReply, RegisterRequest};
use tonic::Code;
use tonic::{
    transport::{Identity, Server, ServerTlsConfig},
    Request, Response, Status,
};

pub mod registration {
    tonic::include_proto!("registration");
}

pub mod auth;
pub mod backend;
pub mod config;

pub struct WgRegistration {
    config: WireguardConfig,
    wg_port: String,
    hooks: RegistrationHooks,
}

impl WgRegistration {
    pub fn new(
        wg_port: &str,
        config_file: &str,
        pre_register_script: Option<&str>,
        post_register_script: Option<&str>,
    ) -> WgRegistration {
        let mut hooks = RegistrationHooks::new();
        if let Some(pre_script) = pre_register_script {
            hooks.pre_register.push_str(pre_script);
        }
        if let Some(post_script) = post_register_script {
            hooks.post_register.push_str(post_script);
        }
        WgRegistration {
            config: WireguardConfig::new(config_file),
            wg_port: wg_port.into(),
            hooks: hooks,
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
        // pre register hook
        match self.hooks.exec_pre_register().await {
            Ok(output) if output.len() > 0 => info!("{}", output),
            Ok(_) => (),
            Err(error) => {
                error!("Pre Register Hook Fail: {}", error);
                return Err(Status::new(Code::Internal, "Internal Server Error"));
            }
        }
        let client_public_key = request.into_inner().public_key;
        // maybe move to non-blocking
        let registration = self
            .config
            .register(client_public_key)
            .await
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
        // post register hook
        match self.hooks.exec_post_register().await {
            Ok(output) if output.len() > 0 => info!("{}", output),
            Ok(_) => (),
            Err(error) => {
                error!("Post Register Hook Fail: {}", error);
                return Err(Status::new(Code::Internal, "Internal Server Error"));
            }
        }
        Ok(Response::new(reply))
    }
}

#[tokio::main]
pub async fn start_server(
    ip: &str,
    port: &str,
    wg_port: &str,
    config_file: &str,
    pre_register: Option<&str>,
    post_register: Option<&str>,
    auth_script: Option<&str>,
    server_tls_cert: Option<&str>,
    server_tls_key: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("starting server on port {}", port);
    let addr = format!("{}:{}", ip, port).parse().unwrap();
    let identity = match (server_tls_cert, server_tls_key) {
        (Some(cert_file), Some(key_file)) => {
            let cert = tokio::fs::read(cert_file).await?;
            let key = tokio::fs::read(key_file).await?;
            Some(Identity::from_pem(cert, key))
        }
        _ => None,
    };

    if identity.is_none() && (server_tls_key.is_some() || server_tls_key.is_some()) {
        bail!("Error setting up tls! Please check certificates");
    }

    let registration = WgRegistration::new(wg_port, config_file, pre_register, post_register);
    let mut server = Server::builder();
    if identity.is_some() {
        info!("using tls");
        server = server.tls_config(ServerTlsConfig::new().identity(identity.unwrap()))?;
    }
    let service = if auth_script.is_some() {
        let static_script = Box::leak(auth_script.unwrap().into());
        RegistrationServer::with_interceptor(registration, interceptor(static_script))
    } else {
        RegistrationServer::new(registration)
    };
    let router = server.add_service(service);
    router.serve(addr).await?;
    Ok(())
}
