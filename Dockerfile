FROM rust:1.49 as builder
WORKDIR /usr/src/stock_exporter
ADD . .
RUN cargo install --path .

FROM debian:buster-slim
ENV RUST_LOG=debug
RUN apt-get update \ 
    && apt-get install -y libssl1.1 ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/stock_exporter /usr/local/bin/stock_exporter