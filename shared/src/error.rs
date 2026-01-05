use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum KmsToolError {
    #[error("failed to Command::new().output()")]
    Io(#[from] std::io::Error),
    #[error("failed to parse command output, stdout: {stdout} status: {status} stderror: {stderr}")]
    StdoutParse {
        stdout: String,
        status: String,
        stderr: String,
    },
    #[error("failed to decode stdout from base64")]
    DecodeError(#[from] base64::DecodeError),
}

#[derive(Debug, thiserror::Error)]
pub enum Aes256GcmError {
    #[error("aes256gcm key was invalid (not 32 bytes)")]
    InvalidLength,
    #[error("encryption operation failed")]
    EncryptionFailed,
    #[error("decryption operation failed")]
    DecryptionFailed,
}

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum VsockEnclaveCreateWalletError {
    #[error("{0}")]
    KmsToolError(String),
    #[error("{0}")]
    Aes256GcmError(String),
}

impl From<KmsToolError> for VsockEnclaveCreateWalletError {
    fn from(e: KmsToolError) -> Self {
        VsockEnclaveCreateWalletError::KmsToolError(e.to_string())
    }
}

impl From<Aes256GcmError> for VsockEnclaveCreateWalletError {
    fn from(e: Aes256GcmError) -> Self {
        VsockEnclaveCreateWalletError::Aes256GcmError(e.to_string())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum VsockReceiveError {
    #[error("failed to stream.read_exact()")]
    Io(#[from] std::io::Error),
    #[error("failed to deserialize cbor")]
    Deserialization(#[from] serde_cbor::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum VsockSendError {
    #[error("failed to stream.write_all()")]
    Io(#[from] std::io::Error),
    #[error("failed to serialize cbor")]
    Serialization(#[from] serde_cbor::Error),
}
