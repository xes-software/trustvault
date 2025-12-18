use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(long)]
    pub proxy_port: String,
}
