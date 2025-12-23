use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};

#[derive(Debug, thiserror::Error)]
pub enum Aes256GcmError {
    #[error("aes256gcm key was invalid (not 32 bytes)")]
    InvalidLength,
    #[error("encryption operation failed")]
    EncryptionFailed,
    #[error("decryption operation failed")]
    DecryptionFailed,
}

pub fn encrypt_private_key_aes256gcm(
    private_key: &[u8; 64],
    encryption_key: &[u8; 32],
    nonce: &[u8; 12],
) -> Result<Vec<u8>, Aes256GcmError> {
    let cipher =
        Aes256Gcm::new_from_slice(encryption_key).map_err(|_| Aes256GcmError::InvalidLength)?;
    let nonce = Nonce::from_slice(nonce);

    let ciphertext = cipher
        .encrypt(nonce, private_key.as_ref())
        .map_err(|_| Aes256GcmError::EncryptionFailed)?;

    return Ok(ciphertext);
}

pub fn decrypt_private_key_aes256gcm(
    ciphertext: &[u8],
    encryption_key: &[u8; 32],
    nonce: &[u8; 12],
) -> Result<Vec<u8>, Aes256GcmError> {
    let cipher =
        Aes256Gcm::new_from_slice(encryption_key).map_err(|_| Aes256GcmError::InvalidLength)?;

    let nonce = Nonce::from_slice(nonce);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| Aes256GcmError::DecryptionFailed)?;

    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_success() {
        let private_key = [42u8; 64];
        let encryption_key = [1u8; 32];
        let nonce = [0u8; 12];

        let ciphertext = encrypt_private_key_aes256gcm(&private_key, &encryption_key, &nonce)
            .expect("encryption should succeed");

        assert_ne!(ciphertext.as_slice(), &private_key[..]);

        let decrypted = decrypt_private_key_aes256gcm(&ciphertext, &encryption_key, &nonce)
            .expect("decryption should succeed");

        assert_eq!(decrypted.len(), 64);
        assert_eq!(decrypted, private_key);
    }

    #[test]
    fn test_decrypt_with_wrong_key_fails() {
        let private_key = [42u8; 64];
        let encryption_key = [1u8; 32];
        let wrong_key = [2u8; 32];
        let nonce = [0u8; 12];

        let ciphertext = encrypt_private_key_aes256gcm(&private_key, &encryption_key, &nonce)
            .expect("encryption unexpectedly failed");

        let result = decrypt_private_key_aes256gcm(&ciphertext, &wrong_key, &nonce);

        assert!(result.is_err());
        match result {
            Err(Aes256GcmError::DecryptionFailed) => {}
            _ => panic!("Expected DecryptionFailed error"),
        }
    }
}
