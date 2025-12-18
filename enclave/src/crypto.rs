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
