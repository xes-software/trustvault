use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(long)]
    pub vsock_port: u32,
}
