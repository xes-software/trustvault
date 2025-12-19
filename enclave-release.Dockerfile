FROM amazonlinux:2023

COPY ./target/release/enclave /usr/local/bin/enclave
COPY aws-nitro-enclaves-sdk-c/bin/kmstool-enclave-cli/kmstool_enclave_cli /usr/local/bin/
COPY aws-nitro-enclaves-sdk-c/bin/kmstool-enclave-cli/libnsm.so /usr/lib/
RUN chmod +x /usr/local/bin/enclave && \
    chmod +x /usr/local/bin/kmstool_enclave_cli


RUN echo "=== Binary check ===" && \
    ls -la /usr/local/bin/enclave && \
    file /usr/local/bin/enclave && \
    ldd /usr/local/bin/enclave 2>&1 || true

CMD ["enclave", "--vsock-port", "3000"]
