use base64::{decode, encode};
use rand_core::OsRng;
use x25519_dalek::{PublicKey, StaticSecret};

//add error handling
pub fn get_public_key(private_key: &str) -> String {
    let decoded_bytes = decode(private_key).unwrap();
    let mut private_bytes = [0u8; 32];
    private_bytes[..decoded_bytes.len()].copy_from_slice(&decoded_bytes);
    let secret = StaticSecret::from(private_bytes);
    let pub_key = PublicKey::from(&secret);
    encode(pub_key.as_bytes())
}

pub fn generate_key_pair() -> (String, String) {
    let secret = StaticSecret::new(OsRng);
    let private_key = encode(secret.to_bytes());
    let public_key = encode(PublicKey::from(&secret).as_bytes());
    (private_key, public_key)
}

#[cfg(test)]
mod tests {

    use super::{generate_key_pair, get_public_key};
    #[test]
    fn test_get_public_key() {
        let (priv_key, pub_key) = generate_key_pair();
        let pub_key_test = get_public_key(&priv_key);
        assert_eq!(pub_key, pub_key_test)
    }
}
