# Nym exchange server Docker image
# Build args:
# - port: set the port on which the server will listen to

# source https://hub.docker.com/_/rust

# Build stage
FROM rust:slim as builder
WORKDIR /opt/server
COPY . .
RUN cargo build --release --locked
 
# Run stage
FROM debian:buster-slim as runner
WORKDIR /opt/server

ARG port=4000
ENV PORT $port

COPY --from=builder /opt/server/target/release/nym-exchange /usr/local/bin/nym-exchange

VOLUME /opt/server/geo_ip/db.mmdb
VOLUME /opt/server/bity/config.json
VOLUME /opt/server/assets

ENV GEOIP_DB_PATH /opt/server/geo_ip/db.mmdb
ENV BITY_CONFIG_PATH /opt/server/bity/config.json
ENV ASSETS_DIRECTORY /opt/server/assets

EXPOSE $port

CMD ["nym-exchange"]
