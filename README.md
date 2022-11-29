# Nym Exchange Interface

Web exchange interface built with [Bity](https://bity.com/) API.
The server side is built with [axum](https://github.com/tokio-rs/axum).

## Install

First provide the required environment variables. For convenience
you can copy over the content of `.env.sample` into a new `.env` file.

Then you need to provide Bity configuration. For convenience you
can copy over the content of `bity_config.sample.json` into a new
`bity_config.json` file. For reference see
https://www.npmjs.com/package/@bity/preact-exchange-client

The server uses [GeoLite2](https://dev.maxmind.com/geoip/geolite2-free-geolocation-data)
database for IPs geolocation (in order to prevent exchange service
access to US american IPs, and to stay compliant with the US law).

To automatically install (and update) the binary database you can
use the provided docker service:

```shell
docker compose up -d
```

## Dev

```shell
cargo run
```

## Build

```shell
cargo build --release
```

## Production

The binary server needs access to:

- `assets` directory containing the website static files
- Bity config file `bity_config.json`
- GeoLite2 country database file

Paths to these files can be provided respectively by the following
env variables: `ASSETS_DIRECTORY`, `BITY_CONFIG_PATH` and
`GEOIP_DB_PATH`.
