use tokio::process::Command;

pub async fn genrandom(
    aws_region: &str,
    aws_access_key_id: &str,
    aws_secret_access_key: &str,
    aws_session_token: &str,
    kms_proxy_port: &str,
    byte_length: &str,
) -> Result<std::process::Output, std::io::Error> {
    return Command::new("kmstool_enclave_cli")
        .arg("genrandom")
        .arg("--region")
        .arg(aws_region)
        .arg("--aws-access-key-id")
        .arg(aws_access_key_id)
        .arg("--aws-secret-access-key")
        .arg(aws_secret_access_key)
        .arg("--aws-session-token")
        .arg(aws_session_token)
        .arg("--proxy-port")
        .arg(kms_proxy_port)
        .arg("--length")
        .arg(byte_length)
        .output()
        .await;
}

pub async fn genkey(
    region: &str,
    access_key_id: &str,
    secret_access_key: &str,
    session_token: &str,
    proxy_port: &str,
    key_id: &str,
    key_spec: &str,
) -> Result<std::process::Output, std::io::Error> {
    return Command::new("kmstool_enclave_cli")
        .arg("genkey")
        .arg("--region")
        .arg(region)
        .arg("--aws-access-key-id")
        .arg(access_key_id)
        .arg("--aws-secret-access-key")
        .arg(secret_access_key)
        .arg("--aws-session-token")
        .arg(session_token)
        .arg("--proxy-port")
        .arg(proxy_port)
        .arg("--key-id")
        .arg(key_id)
        .arg("--key-spec")
        .arg(key_spec)
        .output()
        .await;
}

pub async fn decrypt(
    region: &str,
    access_key_id: &str,
    secret_access_key: &str,
    session_token: &str,
    proxy_port: &str,
    ciphertext: &str,
) -> Result<std::process::Output, std::io::Error> {
    return Command::new("kmstool_enclave_cli")
        .arg("genkey")
        .arg("--region")
        .arg(region)
        .arg("--aws-access-key-id")
        .arg(access_key_id)
        .arg("--aws-secret-access-key")
        .arg(secret_access_key)
        .arg("--aws-session-token")
        .arg(session_token)
        .arg("--proxy-port")
        .arg(proxy_port)
        .arg("--ciphertext")
        .arg(ciphertext)
        .output()
        .await;
}
