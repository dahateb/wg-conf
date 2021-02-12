use self::config::build_config_file;
use crate::crypto::{generate_key_pair, get_public_key};
use http::Uri;
use ini::{Ini, ParseOption};
use registration::registration_client::RegistrationClient;
use registration::RegisterReply;
use registration::RegisterRequest;
use std::str::FromStr;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};
use auth::interceptor;

pub mod registration {
    tonic::include_proto!("registration");
}

pub mod config;
pub mod auth;

#[tokio::main]
pub async fn start_client(
    endpoint: &str,
    netmask: &str,
    config_file: Option<&str>,
    ca_cert: Option<&str>,
    auth_token: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    info!("client mode");
    let uri: Uri = endpoint.parse()?;

    let channel = if ca_cert.is_some() {
        let pem = tokio::fs::read(ca_cert.unwrap()).await?;
        let ca = Certificate::from_pem(pem);

        let tls = ClientTlsConfig::new()
            .ca_certificate(ca)
            .domain_name(uri.host().unwrap());
        debug!("Using tls for host: {}", uri.host().unwrap());
        Channel::builder(uri).tls_config(tls)?.connect().await?
    } else {
        Channel::builder(uri).connect().await?
    };
    let (private_key, public_key) = get_keys(config_file);

    let mut client = RegistrationClient::with_interceptor(channel, interceptor(auth_token.unwrap()));
    let request = tonic::Request::new(RegisterRequest {
        public_key: public_key,
    });    
    let response = client.register_client(request).await?;

    let reply: &RegisterReply = response.get_ref();
    info!("RESPONSE={:?}", reply);

    build_config_file(
        &reply.ipv4_address,
        private_key.as_str(),
        endpoint,
        &reply.public_key,
        &reply.wg_port,
        netmask,
        config_file,
    )?;

    Ok(())
}

fn get_keys(config_file: Option<&str>) -> (String, String) {
    if let Some(ini_file) = config_file {
        let i: Ini = Ini::load_from_file_opt(
            ini_file,
            ParseOption {
                enabled_quote: true,
                enabled_escape: true,
            },
        )
        .map_err(|e| format!("Error loading ini file {} : {}", ini_file, e))
        .unwrap();
        let section = i.section(Some("Interface")).unwrap();
        let private_key = section.get("PrivateKey");
        (
            String::from_str(private_key.unwrap()).unwrap(),
            get_public_key(private_key.unwrap()),
        )
    } else {
        generate_key_pair()
    }
}
