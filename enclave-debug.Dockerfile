FROM alpine:3.23.2

COPY ./target/debug/enclave .

COPY aws-nitro-enclaves-sdk-c/bin/kmstool-enclave-cli/kmstool_enclave_cli /usr/local/bin/
COPY aws-nitro-enclaves-sdk-c/bin/kmstool-enclave-cli/libnsm.so /usr/lib/
RUN chmod +x /usr/local/bin/kmstool_enclave_cli

CMD ["./enclave", "--vsock-port", "3000"]
