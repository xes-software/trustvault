use clap::Parser;
use shared::transport::{
    VsockEnclaveCreateWalletData, VsockEnclaveCreateWalletResponse, VsockHostRequest,
    VsockTransport,
};
use tokio_vsock::{VMADDR_CID_ANY, VsockAddr, VsockListener};

use crate::aes256gcm::encrypt_private_key_aes256gcm;

pub mod aes256gcm;
pub mod cli;
pub mod kmstool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Args::parse();
    let vsock_addr = VsockAddr::new(VMADDR_CID_ANY, args.vsock_port);
    let listener = VsockListener::bind(vsock_addr)
        .expect(&format!("failed to bind vsock on port {}", args.vsock_port));

    loop {
        let (stream, addr) = match listener.accept().await {
            Ok(connection) => connection,
            Err(e) => {
                #[cfg(debug_assertions)]
                eprintln!("failed to accept connection with error: {}", e);
                continue;
            }
        };

        #[cfg(debug_assertions)]
        println!("received vsock connection {} ", addr);

        tokio::spawn(async move {
            let mut transport = VsockTransport::new(stream);

            let request = match transport.receive::<VsockHostRequest>().await {
                Ok(request) => request,
                Err(e) => {
                    // TODO: figure out how best to handle vsock errors instead of silently failing
                    #[cfg(debug_assertions)]
                    eprintln!("failed to receive request: {}", e);
                    return;
                }
            };

            match request {
                VsockHostRequest::CreateWallet {
                    aws_access_key_id,
                    aws_region,
                    aws_secret_access_key,
                    aws_session_token,
                    kms_proxy_port,
                    kms_key_id,
                    aes_gcm_nonce,
                } => {
                    let result = (async || -> VsockEnclaveCreateWalletResponse {
                        let genrandom_output = kmstool::genrandom(
                            aws_region.as_str(),
                            aws_access_key_id.as_str(),
                            aws_secret_access_key.as_str(),
                            aws_session_token.as_str(),
                            kms_proxy_port.as_str(),
                            "64",
                        )
                        .await?;

                        let private_key = &genrandom_output[0];
                        #[cfg(debug_assertions)]
                        eprintln!("kmstool::genrandom() stdout: {:?}", private_key);

                        let [encryption_key_ciphertext, encryption_key_plaintext] =
                            kmstool::genkey(
                                aws_region.as_str(),
                                aws_access_key_id.as_str(),
                                aws_secret_access_key.as_str(),
                                aws_session_token.as_str(),
                                kms_proxy_port.as_str(),
                                kms_key_id.as_str(),
                                "AES-256",
                            )
                            .await?;

                        let private_key_ciphertext = encrypt_private_key_aes256gcm(
                            private_key.as_slice().try_into().unwrap(),
                            encryption_key_plaintext.as_slice().try_into().unwrap(),
                            &aes_gcm_nonce,
                        )?;

                        return Ok(VsockEnclaveCreateWalletData {
                            aes_gcm_nonce: aes_gcm_nonce,
                            encrypted_secret_key: private_key_ciphertext,
                            kms_ciphertext: encryption_key_ciphertext
                                .as_slice()
                                .try_into()
                                .unwrap(),
                            kms_key_id: kms_key_id,
                        });
                    })()
                    .await;

                    let send_result = transport
                        .send::<VsockEnclaveCreateWalletResponse>(&result)
                        .await;

                    if let Err(e) = send_result {
                        // TODO: figure out how best to handle vsock errors instead of silently failing
                        #[cfg(debug_assertions)]
                        eprintln!("failed to send send result: {}", e);
                        return;
                    }
                }
                VsockHostRequest::Sign {} => {

                    let decrypted = kmstool::decrypt(region, access_key_id, secret_access_key, session_token, proxy_port, ciphertext)
                    #[cfg(debug_assertions)]
                    eprintln!("sign is not yet implemented");
                }
            };
        });
    }
}
