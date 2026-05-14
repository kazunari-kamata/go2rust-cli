FROM rust:1-bookworm AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --locked

FROM debian:bookworm-slim

RUN useradd --create-home --shell /usr/sbin/nologin go2rust

COPY --from=builder /app/target/release/go2rust-cli /usr/local/bin/go2rust-cli

USER go2rust
WORKDIR /workspace

ENTRYPOINT ["go2rust-cli"]

