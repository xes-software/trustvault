use clap::Parser;
use shared::transport::{VsockHostRequest, VsockTransport};
use tokio_vsock::{VMADDR_CID_ANY, VsockAddr, VsockListener};

pub mod cli;
pub mod crypto;
pub mod kmstool;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vsock_addr = VsockAddr::new(VMADDR_CID_ANY, 3000);
    let listener = VsockListener::bind(vsock_addr).expect("Unable to bind to vsock port.");

    loop {
        // TODO: handle .accept() error so the program never crashes.
        let (stream, addr) = listener.accept().await.unwrap();

        #[cfg(debug_assertions)]
        println!("received vsock connection {:?} ", addr);

        tokio::spawn(async move {
            let mut transport = VsockTransport::new(stream);
            let request = transport.receive::<VsockHostRequest>().await;

            let response = match request {
                Ok(r) => {
                    return "".to_string();
                }
                Err(e) => {
                    return e.to_string();
                }
            };
        });
    }
}
