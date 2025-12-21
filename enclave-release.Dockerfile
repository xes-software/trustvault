FROM amazonlinux:2023

COPY ./target/aarch64-unknown-linux-musl/release/enclave /usr/local/bin/enclave
COPY aws-nitro-enclaves-sdk-c/bin/kmstool-enclave-cli/kmstool_enclave_cli /usr/local/bin/
COPY aws-nitro-enclaves-sdk-c/bin/kmstool-enclave-cli/libnsm.so /usr/lib/
RUN chmod +x /usr/local/bin/enclave && \
    chmod +x /usr/local/bin/kmstool_enclave_cli

CMD ["enclave", "--vsock-port", "3000"]
