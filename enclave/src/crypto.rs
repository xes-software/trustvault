/// TODO: remove pallas_crypto and use ed25519-dalek crate.
use pallas_crypto::key::ed25519;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct StorableCardanoKey {
    pub encrypted_private_key: String,
    pub encryption_key_ciphertext: String,
    pub encryption_key_nonce: String,
}

impl StorableCardanoKey {
    pub fn serialize_cbor(&self) -> Result<Vec<u8>, serde_cbor::Error> {
        serde_cbor::to_vec(self)
    }
}

pub fn create(bytes: [u8; 64]) {
    let secret_key = ed25519::SecretKeyExtended::from_bytes(bytes).unwrap();
}
