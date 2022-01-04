use self::config::{build_config_file, config_file_exists};
use crate::crypto::{generate_key_pair, get_public_key};
use auth::{interceptor, AuthBuilder};
use http::Uri;
use ini::{Ini, ParseOption};
use registration::registration_client::RegistrationClient;
use registration::RegisterReply;
use registration::RegisterRequest;
use std::str::FromStr;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};

pub mod registration {
    tonic::include_proto!("registration");
}

pub mod auth;
pub mod config;

#[tokio::main]
pub async fn start_client(
    endpoint: &str,
    netmask: &str,
    config_file: Option<&str>,
    ca_cert: Option<&str>,
    auth: AuthBuilder,
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

    let (private_key, public_key) = if config_file_exists(config_file) {
        get_keys_from_file(config_file.unwrap())
    } else {
        generate_key_pair()
    };

    let request = tonic::Request::new(RegisterRequest {
        public_key: public_key,
    });

    let response = if auth.has_authentication() {
        RegistrationClient::with_interceptor(channel, interceptor(auth))
            .register_client(request)
            .await?
    } else {
        RegistrationClient::new(channel)
            .register_client(request)
            .await?
    };

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

fn get_keys_from_file(ini_file: &str) -> (String, String) {
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
}
