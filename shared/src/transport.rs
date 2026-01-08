use crate::error::{VsockEnclaveCreateWalletError, VsockReceiveError, VsockSendError};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_vsock::VsockStream;

pub struct VsockTransport {
    stream: VsockStream,
}

impl VsockTransport {
    pub fn new(stream: VsockStream) -> Self {
        return Self { stream };
    }

    pub async fn receive<T: for<'de> Deserialize<'de>>(&mut self) -> Result<T, VsockReceiveError> {
        let mut len_bytes = [0u8; 4];
        self.stream.read_exact(&mut len_bytes).await?;
        let len = u32::from_be_bytes(len_bytes) as usize;
        let mut buf = vec![0u8; len];
        self.stream.read_exact(&mut buf).await?;
        let message: T = serde_cbor::from_slice(&buf)?;
        return Ok(message);
    }

    pub async fn send<T: Serialize>(&mut self, message: &T) -> Result<(), VsockSendError> {
        let cbor_bytes = serde_cbor::to_vec(message)?;
        let len = cbor_bytes.len() as u32;
        self.stream.write_all(&len.to_be_bytes()).await?;
        self.stream.write_all(&cbor_bytes).await?;
        self.stream.flush().await?;
        return Ok(());
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum VsockHostRequest {
    CreateWallet {
        aws_region: String,
        aws_access_key_id: String,
        aws_secret_access_key: String,
        aws_session_token: String,
        kms_proxy_port: String,
        kms_key_id: String,
        aes_gcm_nonce: [u8; 12],
    },
    Sign {
        aws_region: String,
        aws_access_key_id: String,
        aws_secret_access_key: String,
        aws_session_token: String,
        kms_proxy_port: String,
        kms_key_id: String,
        aes_gcm_nonce: [u8; 12],
        encrypted_secret_key: Vec<u8>,
        kms_ciphertext: Vec<u8>,
        signature_scheme: SignatureScheme,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VsockEnclaveCreateWalletData {
    pub encrypted_secret_key: Vec<u8>,
    pub aes_gcm_nonce: [u8; 12],
    pub kms_ciphertext: [u8; 32],
    pub kms_key_id: String,
}

pub type VsockEnclaveCreateWalletResponse =
    Result<VsockEnclaveCreateWalletData, VsockEnclaveCreateWalletError>;

#[derive(Serialize, Deserialize, Debug)]
pub enum SignatureScheme {
    Secp256k1,
    Ed25519,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VsockEnclaveSignData {}

pub type VsockEnclaveSignResponse = Result<VsockEnclaveSignData, VsockEnclaveCreateWalletError>;
