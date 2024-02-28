# Nym exchange server Docker image
# Build args:
# - port: set the port on which the server will listen to

# source https://hub.docker.com/_/rust

# Build stage
# This configuration is for arm64 
FROM --platform=linux/arm64 rust:slim as builder
WORKDIR /opt/server
COPY . .

RUN rustup target add aarch64-unknown-linux-musl && \
    cargo build --release --locked --target aarch64-unknown-linux-musl

# Run stage
FROM --platform=linux/arm64 debian:bullseye-slim as runner
WORKDIR /opt/server

ARG port=4000
ENV PORT $port

COPY --from=builder /opt/server/target/aarch64-unknown-linux-musl/release/nym-exchange /usr/local/bin/nym-exchange

VOLUME /opt/server/geo_ip/db.mmdb
VOLUME /opt/server/bity/config.json
VOLUME /opt/server/assets

ENV GEOIP_DB_PATH /opt/server/geo_ip/db.mmdb
ENV BITY_CONFIG_PATH /opt/server/bity/config.json
ENV ASSETS_DIRECTORY /opt/server/assets

EXPOSE $port

CMD ["nym-exchange"]