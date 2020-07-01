use x25519_dalek::{StaticSecret, PublicKey};
use base64::{encode, decode};

//add error handling
pub fn get_public_key(private_key: &str) -> String{
    let decoded_bytes = decode(private_key).unwrap();
    let mut private_bytes = [0u8; 32];
    private_bytes[..decoded_bytes.len()].copy_from_slice(&decoded_bytes) ;
    let secret = StaticSecret::from(private_bytes);
    let pub_key = PublicKey::from(&secret);
    encode(pub_key.as_bytes())
}