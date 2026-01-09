use base64::prelude::*;
use shared::error::KmsToolError;
use tokio::process::Command;

pub struct KmsGenkeyOutput {
    pub ciphertext: Vec<u8>,
    pub plaintext: Vec<u8>,
}

pub async fn genrandom(
    aws_region: &str,
    aws_access_key_id: &str,
    aws_secret_access_key: &str,
    aws_session_token: &str,
    kms_proxy_port: &str,
    byte_length: &str,
) -> Result<[Vec<u8>; 1], KmsToolError> {
    let result = Command::new("kmstool_enclave_cli")
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
        .await?;
    let parsed = parse_output(["PLAINTEXT: "], &result)?;
    return Ok(parsed);
}

pub async fn genkey(
    region: &str,
    access_key_id: &str,
    secret_access_key: &str,
    session_token: &str,
    proxy_port: &str,
    key_id: &str,
    key_spec: &str,
) -> Result<[Vec<u8>; 2], KmsToolError> {
    let result = Command::new("kmstool_enclave_cli")
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
        .await?;
    let parsed = parse_output(["CIPHERTEXT: ", "PLAINTEXT: "], &result)?;
    return Ok(parsed);
}

pub async fn decrypt(
    region: &str,
    access_key_id: &str,
    secret_access_key: &str,
    session_token: &str,
    proxy_port: &str,
    ciphertext_base64: &str,
) -> Result<[Vec<u8>; 1], KmsToolError> {
    let result = Command::new("kmstool_enclave_cli")
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
        .arg(ciphertext_base64)
        .output()
        .await?;
    let parsed = parse_output(["PLAINTEXT: "], &result)?;
    return Ok(parsed);
}

fn parse_output<const N: usize>(
    ordered_line_prefixes: [&str; N],
    output: &std::process::Output,
) -> Result<[Vec<u8>; N], KmsToolError> {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let decode_field = |prefix: &str| -> Result<Vec<u8>, KmsToolError> {
        stdout
            .lines()
            .find_map(|line| line.trim().strip_prefix(prefix))
            .ok_or_else(|| KmsToolError::StdoutParse {
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                status: output.status.to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            })
            .and_then(|s| BASE64_STANDARD.decode(s.trim()).map_err(Into::into))
    };
    return ordered_line_prefixes
        .into_iter()
        .map(decode_field)
        .collect::<Result<Vec<_>, _>>()?
        .try_into()
        .map_err(|_| KmsToolError::StdoutParse {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            status: output.status.to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::process::ExitStatusExt;
    use std::process::{ExitStatus, Output};

    #[test]
    fn test_successful_parse_output_decrypt_or_genrandom() {
        let stdout = format!("PLAINTEXT: {}\n", BASE64_STANDARD.encode(b"Hello World"));
        let stderr = "Some warning message\n";
        let output = Output {
            status: ExitStatus::from_raw(0),
            stdout: stdout.as_bytes().to_vec(),
            stderr: stderr.as_bytes().to_vec(),
        };

        let result = parse_output(["PLAINTEXT: "], &output).unwrap();

        assert_eq!(result[0], b"Hello World");
    }

    #[test]
    fn test_successful_parse_output_genkey() {
        let stdout = format!(
            "CIPHERTEXT: {}\nPLAINTEXT: {}\n",
            BASE64_STANDARD.encode(b"ciphertext"),
            BASE64_STANDARD.encode(b"secretdata!")
        );
        let stderr = "Some warning message\n";
        let output = Output {
            status: ExitStatus::from_raw(256),
            stdout: stdout.as_bytes().to_vec(),
            stderr: stderr.as_bytes().to_vec(),
        };

        let result = parse_output(["CIPHERTEXT: ", "PLAINTEXT: "], &output).unwrap();

        assert_eq!(result[0], b"ciphertext");
        assert_eq!(result[1], b"secretdata!");
    }

    #[test]
    fn test_unsuccessful_parse_output_no_stdout() {
        let stdout = "";
        let stderr = "Some warning message\n";
        let output = Output {
            status: ExitStatus::from_raw(256),
            stdout: stdout.as_bytes().to_vec(),
            stderr: stderr.as_bytes().to_vec(),
        };

        let result = parse_output(["PLAINTEXT: "], &output);

        match result {
            Err(err) => match err {
                KmsToolError::StdoutParse { stderr, .. } => {
                    assert_eq!(stderr, "Some warning message\n");
                }
                _ => {
                    panic!("expect KmsToolError::StdoutParse variant");
                }
            },
            Ok(_ok) => {
                panic!("expected error got ok");
            }
        }
    }

    #[test]
    fn test_successful_parse_output_malformed_stdout() {
        let stdout = format!(
            "PLAINTEXT: {}\nCIPHERTEXT: {}\n",
            BASE64_STANDARD.encode(b"secretdata!"),
            BASE64_STANDARD.encode(b"ciphertext"),
        );
        let stderr = "Some warning message\n";
        let output = Output {
            status: ExitStatus::from_raw(256),
            stdout: stdout.as_bytes().to_vec(),
            stderr: stderr.as_bytes().to_vec(),
        };

        let result = parse_output(["CIPHERTEXT: ", "PLAINTEXT: "], &output).unwrap();
        assert_eq!(result[0], b"ciphertext");
        assert_eq!(result[1], b"secretdata!");
    }
}
