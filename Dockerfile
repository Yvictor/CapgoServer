FROM rust:1.68-buster as builder
WORKDIR /home/capgo
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release 
# RUN rustup target add x86_64-unknown-linux-musl && cargo build --release && cargo install --target x86_64-unknown-linux-musl --path .
CMD ["./target/release/CapgoServer"]

# FROM debian:buster
# COPY --from=builder /home/capgo/target/release/CapgoServer .
# CMD ["./CapgoServer"]