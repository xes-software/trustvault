# deploy with ec2 instance:

Create an ec2 instance with nitro enclaves enabled with amazon linux 2023 (m7g.large)[$0.0816 per hour on demand].

```bash
cd ~/
```

# Fetch repository

```bash
git clone --recurse-submodules https://github.com/xes-software/trustvault.git
```

# Build KMS Tool Enclave CLI

```bash
cd trustvault/aws-nitro-enclaves-sdk-c/bin/kmstool-enclave-cli/
./build.sh
cd ~/
```

# Build application (debug)
```bash
cd ./trustvault
cargo build
cd ~/
```

# Build docker image

```bash
cd ./trustvault
docker build -f enclave-debug.Dockerfile -t enclave .
```

# Build enclave image

```bash
cd ~/
nitro-cli build-enclave --docker-uri enclave --output-file ./trustvault/enclave.eif
```

# Start enclave

```bash
nitro-cli run-enclave --cpu-count 1 --memory 2000 --enclave-cid 16 --eif-path ./trustvault/enclave.eif --debug-mode
```

# Start server to talk to enclave

```bash
./trustvault/target/debug/host --aws-region us-east-1
```
