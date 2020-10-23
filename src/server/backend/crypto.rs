use base64::{decode, encode_config};
use x25519_dalek::{PublicKey, StaticSecret};

//add error handling
pub fn get_public_key(private_key: &str) -> String {
    let decoded_bytes = decode(private_key).unwrap();
    let mut private_bytes = [0u8; 32];
    private_bytes[..decoded_bytes.len()].copy_from_slice(&decoded_bytes);
    let secret = StaticSecret::from(private_bytes);
    let pub_key = PublicKey::from(&secret);
    encode_config(pub_key.as_bytes(), base64::URL_SAFE)
}
