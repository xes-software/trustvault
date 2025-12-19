use clap::Parser;
use shared::transport::{VsockHostRequest, VsockTransport};
use tokio_vsock::{VMADDR_CID_ANY, VsockAddr, VsockListener};

pub mod cli;
pub mod crypto;
pub mod kmstool;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Args::parse();
    let vsock_addr = VsockAddr::new(VMADDR_CID_ANY, args.vsock_port);
    let listener = match VsockListener::bind(vsock_addr) {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!(
                "✗ Failed to bind vsock (this is expected outside enclave): {}",
                e
            );
            eprintln!("Running in test mode...");
            // For docker testing only
            std::thread::sleep(std::time::Duration::from_secs(5));
            panic!("Error");
        }
    };

    loop {
        let (stream, addr) = match listener.accept().await {
            Ok(connection) => connection,
            Err(e) => {
                #[cfg(debug_assertions)]
                eprintln!("failed to accept connection: {}", e);
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
                                #[cfg(debug_assertions)]
                                eprintln!("kmstool::genrandom() stdout: {:?}", output.stdout);

                                eprintln!("kmstool::genrandom() stderr: {:?}", output.stderr);

                                eprintln!("kmstool::genrandom() status: {:?}", output.status);
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
