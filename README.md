# deploy with ec2 instance:

Create an ec2 instance with nitro enclaves enabled with amazon linux 2023 (m7g.large)[$0.0816 per hour on demand].

```bash
cd ~/

sudo mkdir -p /etc/nitro_enclaves
cat <<'EOF' | sudo tee /etc/nitro_enclaves/allocator.yaml > /dev/null
---
memory_mib: 2000
cpu_count: 1
EOF

sudo dnf update -y
sudo dnf install -y \
    gcc \
    gcc-c++ \
    make \
    cmake \
    clang \
    pkg-config \
    openssl-devel \
    git

curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
git clone --recurse-submodules https://github.com/xes-software/trustvault.git
source $HOME/.cargo/env
rustup target add aarch64-unknown-linux-musl
cd ./trustvault
cargo build --target=aarch64-unknown-linux-gnu -p host 

sudo dnf install aws-nitro-enclaves-cli -y
sudo dnf install aws-nitro-enclaves-cli-devel -y

sudo usermod -aG ne ssm-user
sudo usermod -aG docker ssm-user

sudo systemctl enable --now docker
```

# Terminate shell (part 2)


```bash
cd ~/trustvault/aws-nitro-enclaves-sdk-c/bin/kmstool-enclave-cli/
./build.sh

cd ~/trustvault
docker build -f enclave-debug.Dockerfile -t enclave .
cd ~/

sudo systemctl enable --now nitro-enclaves-allocator.service
sudo systemctl start nitro-enclaves-vsock-proxy.service
sudo systemctl enable nitro-enclaves-vsock-proxy.service

nitro-cli build-enclave --docker-uri enclave --output-file ./trustvault/enclave.eif
nitro-cli run-enclave --cpu-count 1 --memory 2000 --enclave-cid 16 --eif-path ./trustvault/enclave.eif --debug-mode 
```

# Start server to talk to enclave

```bash
./trustvault/target/debug/host --aws-region us-east-1
```
