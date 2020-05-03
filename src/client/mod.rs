use registration::registration_client::RegistrationClient;
use registration::RegisterRequest;

pub mod registration {
    tonic::include_proto!("registration");
}

#[tokio::main]
pub async fn start_client(endpoint: &str, client_public_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("client mode");
    let mut client = RegistrationClient::connect(endpoint.to_string()).await?;

    let request = tonic::Request::new(RegisterRequest {
        public_key: client_public_key.into(),
    });

    let response = client.register_client(request).await?;
    info!("RESPONSE={:?}", response);

    Ok(())
}
