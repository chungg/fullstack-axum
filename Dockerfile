FROM rust:latest as builder
WORKDIR app

COPY Cargo.toml Cargo.lock /app/

# https://stackoverflow.com/a/76929661
RUN mkdir src && \
    echo 'fn main() {\nprintln!("Hello, world!");\n}' > src/main.rs && \
    cargo build --release && \
    cargo clean --package $(awk '/name/ {gsub(/"/,""); print $3}' Cargo.toml | sed ':a;N;$!ba;s/\n//g' | tr -d '\r') && \
    rm -rf src 

COPY src /app/src
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR app
COPY --from=builder /app/target/release/fullstack-axum /usr/local/bin
