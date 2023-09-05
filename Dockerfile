# build stage for rust application
FROM rust:1.65.0 as rust-builder
WORKDIR /usr/src/cyborgai_rust
COPY ./cyborgai_rust/Cargo.toml ./Cargo.toml
RUN mkdir -p ./src
#     echo "fn main() {println!(\"if you see this, the build broke\")}" > ./src/main.rs && \
#     cargo build --release
# RUN rm ./src/main.rs
COPY ./cyborgai_rust/src/main.rs ./src/main.rs
RUN cargo build --release

# debian buster-slim as base image
FROM debian:buster-slim as stage-2

# installing basic tools for network troubleshooting
RUN apt-get update && \
    apt-get install -y wget procps && \
    rm -rf /var/lib/apt/lists/*

# setting working directory
WORKDIR /app

# copying artifact from builder to this new stage
COPY --from=rust-builder /usr/src/cyborgai_rust/target/release/cyborgai_rust ./cyborgai_rust

# installing IPFS
RUN wget https://dist.ipfs.io/go-ipfs/v0.20.0/go-ipfs_v0.20.0_linux-amd64.tar.gz && \
    tar xvfz go-ipfs_v0.20.0_linux-amd64.tar.gz && \
    mv go-ipfs/ipfs /usr/local/bin/ && \
    rm -rf go-ipfs go-ipfs_v0.20.0_linux-amd64.tar.gz

#Initialize IPFS and set the configuration
RUN ipfs init && \
    ipfs config --json --config /data/ipfs/config Swarm.RelayClient.Enabled true && \
    ipfs config --json --config /data/ipfs/config Swarm.EnableHolePunching true && \
    ipfs config --json --config /data/ipfs/config Addresses.Gateway '["/ip4/0.0.0.0/tcp/8080","/ip6/::/tcp/8080"]' && \
    ipfs config --json --config /data/ipfs/config Addresses.API '["/ip4/0.0.0.0/tcp/5001", "/ip6/::/tcp/5001"]' && \
    ipfs config --json --config /data/ipfs/config Swarm.ConnMgr.HighWater 100 && \
    ipfs config --json --config /data/ipfs/config Swarm.ConnMgr.LowWater 50 &&\
    ipfs config --config /data/ipfs/config Swarm.ResourceMgr.Enabled --json true



# Create start script for running IPFS and rust application in order.
RUN echo "#!/bin/sh\n./cyborgai_rust &\nipfs daemon \n" > start.sh && chmod +x start.sh
EXPOSE 8081 4001 5001 8080
# Running the shell script as the main process  
CMD ./start.sh