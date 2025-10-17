FROM rust:1.80 as builder
WORKDIR /app
COPY Cargo.toml ./
RUN mkdir -p src && echo "fn main(){}" > src/main.rs && cargo build --release && rm -rf target/release/deps/frigate-agent*
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN useradd -m app
USER app
WORKDIR /home/app
COPY --from=builder /app/target/release/frigate-agent /usr/local/bin/app
ENV RUST_LOG=info
EXPOSE 8080
CMD ["/usr/local/bin/app"]