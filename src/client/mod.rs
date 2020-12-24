use crate::server::backend;

use self::config::{build_config_file, generate_key_pair};
use backend::crypto::get_public_key;
use ini::{Ini, ParseOption};
use registration::registration_client::RegistrationClient;
use registration::RegisterReply;
use registration::RegisterRequest;
use std::str::FromStr;

pub mod registration {
    tonic::include_proto!("registration");
}

pub mod config;

#[tokio::main]
pub async fn start_client(
    endpoint: &str,
    netmask: &str,
    config_file: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("client mode");
    let (private_key, public_key) = if let Some(ini_file) = config_file {
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
    };
    let mut client = RegistrationClient::connect(endpoint.to_string()).await?;
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
