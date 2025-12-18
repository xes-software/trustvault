use clap::Parser;

pub mod cli;
pub mod crypto;
pub mod kmstool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Args::parse();

    let access_key_id = "";
    let secret_access_key = "";
    let session_token = "";

    return Ok(());
}
