FROM rust:1.80 as builder
WORKDIR /app
COPY Cargo.toml ./
RUN mkdir -p src && echo "fn main(){}" > src/main.rs && cargo build --release && rm -rf target/release/deps/frigate_cli*
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN useradd -m app
USER app
WORKDIR /home/app
COPY --from=builder /app/target/release/frigate-cli /usr/local/bin/app
CMD ["/usr/local/bin/app", "--broker-host", "localhost", "--broker-port", "1883", "--topic", "frigate/events"]