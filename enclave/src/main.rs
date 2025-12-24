use clap::Parser;
use shared::transport::{VsockEnclaveCreateWalletResponse, VsockHostRequest, VsockTransport};
use tokio_vsock::{VMADDR_CID_ANY, VsockAddr, VsockListener};

use crate::aes256gcm::encrypt_private_key_aes256gcm;

pub mod aes256gcm;
pub mod cli;
pub mod kmstool;

#[tokio::main(flavor = "current_thread")]
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
            match transport.receive::<VsockHostRequest>().await {
                Ok(r) => match r {
                    VsockHostRequest::CreateWallet {
                        aws_access_key_id,
                        aws_region,
                        aws_secret_access_key,
                        aws_session_token,
                        kms_proxy_port,
                        kms_key_id,
                        nonce,
                    } => {
                        let genrandom_output = kmstool::genrandom(
                            aws_region.as_str(),
                            aws_access_key_id.as_str(),
                            aws_secret_access_key.as_str(),
                            aws_session_token.as_str(),
                            kms_proxy_port.as_str(),
                            "64",
                        )
                        .await;

                        match genrandom_output {
                            Ok(output) => {
                                let private_key = &output[0];
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
                                    .await
                                    .expect("kmstool::genkey() failed unexpectedly");

                                let private_key_ciphertext = encrypt_private_key_aes256gcm(
                                    private_key.as_slice().try_into().unwrap(),
                                    encryption_key_plaintext.as_slice().try_into().unwrap(),
                                    &nonce,
                                )
                                .unwrap();

                                let res = VsockEnclaveCreateWalletResponse {
                                    aes_gcm_nonce: nonce,
                                    encrypted_secret_key: private_key_ciphertext,
                                    kms_ciphertext: encryption_key_ciphertext
                                        .as_slice()
                                        .try_into()
                                        .unwrap(),
                                    kms_key_id: kms_key_id,
                                };
                                transport
                                    .send::<VsockEnclaveCreateWalletResponse>(&res)
                                    .await
                                    .expect("failed to send transport response");
                            }
                            Err(e) => {
                                #[cfg(debug_assertions)]
                                eprintln!("kmstool::genrandom() resulted in an error: {}", e);
                            }
                        }
                    }
                    VsockHostRequest::Sign => {
                        #[cfg(debug_assertions)]
                        eprintln!("sign is not yet implemented");
                    }
                },
                Err(e) => {
                    #[cfg(debug_assertions)]
                    eprintln!("error handling connection: {}", e);
                }
            };
        });
    }
}
