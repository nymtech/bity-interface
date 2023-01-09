// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod client_config;
mod error;
mod ip_check;
mod shutdown;
mod utils;

use anyhow::Context;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, get_service};
use axum::{middleware, Json, Router};
use client_config::ClientConfig;
use dotenv::dotenv;
use ip_check::ip_check;
use maxminddb::Reader;
use shutdown::shutdown_signal;
use std::{env, io};
use std::{net::SocketAddr, sync::Arc};
use tokio::fs;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::{debug, info, instrument};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::utils::read_env_var;

const DEFAULT_DATABASE_PATH: &str = "geo_ip/GeoLite2-Country.mmdb";
const DEFAULT_BITY_CONFIG_PATH: &str = "bity_config.json";
const DEFAULT_ASSETS_DIRECTORY: &str = "assets";
const LOG_LEVELS: &str = "nym_exchange=debug,tower_http=debug";

#[derive(Debug)]
pub struct AppState {
    geoip_db: Reader<Vec<u8>>,
    bity_config: ClientConfig,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    // Tracing init
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG")
                .map(|v| {
                    if v.is_empty() {
                        return LOG_LEVELS.into();
                    }
                    v
                })
                .unwrap_or_else(|_| LOG_LEVELS.into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let port: u16 = env::var("PORT")
        .expect("Environment variable PORT not set")
        .parse()
        .with_context(|| "Environment variable PORT invalid value")?;

    let bity_config_path = read_env_var("BITY_CONFIG_PATH", DEFAULT_BITY_CONFIG_PATH);
    debug!("Reading Bity config from {}", &bity_config_path);
    let bity_config: ClientConfig = serde_json::from_slice(
        &fs::read(&bity_config_path)
            .await
            .with_context(|| format!("Fail to read {}", &bity_config_path))?,
    )
    .with_context(|| format!("Fail to parse client config {}", &bity_config_path))?;

    let db_path = read_env_var("GEOIP_DB_PATH", DEFAULT_DATABASE_PATH);
    debug!("Loading GeoLite2 database {}", &db_path);
    let reader = Reader::open_readfile(&db_path)
        .with_context(|| format!("Fail to open GeoLite2 database file {}", db_path))?;

    let state = Arc::new(AppState {
        geoip_db: reader,
        bity_config,
    });

    let mut assets_dir = read_env_var("ASSETS_DIRECTORY", DEFAULT_ASSETS_DIRECTORY);
    assets_dir = assets_dir.trim_end_matches('/').into();
    info!("Serving files from {} directory", assets_dir);

    // Router setup
    // index.html is located in $ASSETS_DIRECTORY
    // GET /            serves index.html
    // GET /assets/*    serves the corresponding static file from $ASSETS_DIRECTORY/*
    // GET /config      replies with client config in JSON
    // GET /403.html    serves $ASSETS_DIRECTORY/403.html
    // GET /*           fallback to /assets/*
    // GET /assets/file_doesnt_exist fallback to index.html (no 404)
    let serve_dir = ServeDir::new(&assets_dir)
        .not_found_service(ServeFile::new(format!("{}/index.html", assets_dir)));
    let serve_dir = get_service(serve_dir)
        .handle_error(handle_error)
        .route_layer(middleware::from_fn_with_state(state.clone(), ip_check));

    let app = Router::new()
        .route("/config", get(get_config))
        .route(
            "/403.html",
            get_service(ServeFile::new(format!("{}/403.html", assets_dir)))
                .handle_error(handle_error),
        )
        .nest_service("/assets", serve_dir.clone())
        .fallback_service(serve_dir)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("❱ listening on {} ❰", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

#[instrument(skip_all, level = "info")]
pub async fn get_config(State(state): State<Arc<AppState>>) -> Json<ClientConfig> {
    Json(state.bity_config.clone())
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
