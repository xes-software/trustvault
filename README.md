# deploy with ec2 instance:

Create an ec2 instance with nitro enclaves enabled with amazon linux 2023 (m7g.large)[$0.0816 per hour on demand].

```bash
cd ~
```

# Install nitro enclaves functionality

```bash
sudo dnf install aws-nitro-enclaves-cli -y
sudo dnf install aws-nitro-enclaves-cli-devel -y
sudo usermod -aG ne ssm-user
sudo usermod -aG docker ssm-user
```

# Install git

```bash
sudo dnf install -y git
git clone --recurse-submodules https://github.com/xes-software/trustvault.git
```

# Install rustc and needed deps

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo dnf install -y gcc gcc-c++ make cmake clang pkg-config openssl-devel
```

# Set the allocator and start nitro enclaves allocator

```bash
sudo vim /etc/nitro_enclaves/allocator.yaml
```

In configuration file, update `memory_mib: 5000`, and `cpu_count: 1`

```bash
sudo systemctl enable --now nitro-enclaves-allocator.service
```

# Start the vsock proxy for KMS communication

```bash
sudo systemctl start nitro-enclaves-vsock-proxy.service
sudo systemctl enable nitro-enclaves-vsock-proxy.service
```

# Start docker

```bash
sudo systemctl enable --now docker
```

# Exit Terminal

Close the instance and start again.

# Build KMS Tool Enclave CLI

```bash
cd ~/
cd trustvault/aws-nitro-enclaves-sdk-c/bin/kmstool-enclave-cli/
./build.sh
cd ~/
```

# Build application (debug)
```bash
source $HOME/.cargo/env
cd ./trustvault
cargo build
cd ~/
```

# Build docker image

```bash
docker build ./trustvault/enclave-debug.Dockerfile -t enclave
```

# Build enclave image

```bash
nitro-cli build-enclave --docker-uri enclave --output-file enclave.eif
```

# Start enclave

```bash
nitro-cli run-enclave --cpu-count 1 --memory 5000 --enclave-cid 16 --eif-path enclave.eif --debug-mode
```

# Start server to talk to enclave

```bash
/home/ssm-user/.deno/bin/deno run -A ./test-nitro-enclaves/client.ts
```
