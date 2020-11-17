use self::config::{build_config_file, generate_key_pair};
use registration::registration_client::RegistrationClient;
use registration::RegisterReply;
use registration::RegisterRequest;

pub mod registration {
    tonic::include_proto!("registration");
}

pub mod config;

#[tokio::main]
pub async fn start_client(endpoint: &str, netmask: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("client mode");
    let mut client = RegistrationClient::connect(endpoint.to_string()).await?;
    let (private_key, public_key) = generate_key_pair();
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
    )?;

    Ok(())
}
