use base64::encode_config;
use ini::Ini;
use rand_core::OsRng;
use x25519_dalek::{PublicKey, StaticSecret};

pub fn build_config_file(
    address: &str,
    private_key: &str,
    peer_endpoint: &str,
    peer_public_key: &str,
) {
    let mut conf = Ini::new();
    conf.with_section(Some("Interface"))
        .set("Address", address)
        .set("PrivateKey", private_key)
        .set("DNS", "8.8.8.8");
    conf.with_section(Some("Peer"))
        .set("Endpoint", peer_endpoint)
        .set("PublicKey", peer_public_key);
    conf.write_to_file("conf.ini").unwrap();
}

pub fn generate_key_pair() -> (String, String) {
    let secret = StaticSecret::new(OsRng);
    let private_key = encode_config(secret.to_bytes(), base64::URL_SAFE);
    let public_key = encode_config(PublicKey::from(&secret).as_bytes(), base64::URL_SAFE);
    (private_key, public_key)
}
