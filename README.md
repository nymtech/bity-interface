# Nym Exchange Interface

Web exchange interface built with [Bity](https://bity.com/) API.

## Install

First provide the required environment variables. For convenience
you can copy over the content of `.env.sample` into a new `.env` file.

The server uses [GeoLite2](https://dev.maxmind.com/geoip/geolite2-free-geolocation-data)
database for IPs geolocation (in order to prevent exchange service
access to US american IPs, and to stay compliant with the US law).

To automatically install (and update) the binary database you can
use the provided docker service:

```shell
docker compose up -d
```

Finally provide the Bity [config](https://www.npmjs.com/package/@bity/preact-exchange-client)
properties you want in `bity_config.json` file.

## Dev

```shell
cargo run
```

## Build

```shell
cargo build --release
```
