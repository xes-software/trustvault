use aws_config::{BehaviorVersion, Region};
use aws_sdk_sts::Client as StsClient;
use clap::Parser;
use shared::transport::{VsockEnclaveCreateWalletResponse, VsockHostRequest, VsockTransport};
use tokio_vsock::{VsockAddr, VsockStream};

#[derive(Parser)]
pub struct Args {
    #[arg(long)]
    pub aws_region: String,
    #[arg(long)]
    pub vsock_port: u32,
    #[arg(long)]
    pub enclave_cid: u32,
    #[arg(long)]
    pub kms_proxy_port: String,
    #[arg(long)]
    pub kms_key_id: String,
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
        .role_session_name(format!(
            "trustvault-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("failed to get system time")
                .as_secs()
        ))
        .duration_seconds(3600)
        .send()
        .await
        .expect("failed to obtain sts assume role");

    println!("{:?}", response.credentials());

    let addr = VsockAddr::new(args.enclave_cid, args.vsock_port);

    let stream = VsockStream::connect(addr)
        .await
        .expect("failed to connect to 16 3000");

    let mut transport = VsockTransport::new(stream);

    let request = VsockHostRequest::CreateWallet {
        aws_region: args.aws_region,
        aws_access_key_id: response.credentials().unwrap().access_key_id.clone(),
        aws_secret_access_key: response.credentials().unwrap().secret_access_key.clone(),
        aws_session_token: response.credentials().unwrap().session_token.clone(),
        kms_proxy_port: args.kms_proxy_port,
        kms_key_id: args.kms_key_id,
    };

    transport
        .send::<VsockHostRequest>(&request)
        .await
        .expect("failed to send transport layer");

    let response = transport
        .receive::<VsockEnclaveCreateWalletResponse>()
        .await
        .expect("failed to recieve response");
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
