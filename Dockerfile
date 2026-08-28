FROM rust:1.98 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/unifi-vpn-config /app/unifi-vpn-config

ENV PORT=8080
ENV UNIFI_USERNAME=""
ENV UNIFI_BASEURL=""
ENV UNIFI_PASSWORD=""

EXPOSE 8080
ENTRYPOINT ["/app/unifi-vpn-config"]
