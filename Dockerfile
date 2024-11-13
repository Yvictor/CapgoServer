FROM rust:1.82-slim AS builder
WORKDIR /home/capgo
COPY Cargo.toml Cargo.lock ./
COPY src ./src
# RUN apt update && apt install -y libssl-dev pkg-config
RUN cargo build --release 
# RUN rustup target add x86_64-unknown-linux-musl && cargo build --release && cargo install --target x86_64-unknown-linux-musl --path .
CMD ["./target/release/CapgoServer"]

FROM debian:stable-slim
COPY --from=builder /home/capgo/target/release/CapgoServer .
CMD ["./CapgoServer"]