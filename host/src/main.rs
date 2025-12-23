use aws_config::{BehaviorVersion, Region};
use aws_sdk_sts::Client as StsClient;
use clap::Parser;
use shared::transport;

#[derive(Parser)]
pub struct Args {
    #[arg(long)]
    pub aws_region: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Args::parse();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(args.aws_region.clone()))
        .load()
        .await;

    let sts_client = StsClient::new(&config);

    let identity = sts_client.get_caller_identity().send().await.unwrap();

    let arn = identity.arn().unwrap().to_string();

    let role_arn = convert_to_role_arn(&arn);

    let response = sts_client
        .assume_role()
        .role_arn(&role_arn)
        .duration_seconds(3600)
        .send()
        .await
        .expect("failed to obtain sts assume role");

    println!("{:?}", response);
}

fn convert_to_role_arn(assumed_role_arn: &str) -> String {
    let parts: Vec<&str> = assumed_role_arn.split(':').collect();
    if parts.len() < 6 {
        panic!("Above 6");
    }

    let account_id = parts[4];
    let resource_parts: Vec<&str> = parts[5].split('/').collect();
    if resource_parts.len() < 2 {
        panic!("less than 2");
    }

    let role_name = resource_parts[1];
    let role_arn = format!("arn:aws:iam::{}:role/{}", account_id, role_name);

    return role_arn;
}
