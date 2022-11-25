mod error;
mod ip_check;
mod shutdown;

use anyhow::Context;
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, get_service};
use axum::{middleware, Extension, Router};
use dotenv::dotenv;
use ip_check::ip_check;
use maxminddb::Reader;
use shutdown::shutdown_signal;
use std::{env, io};
use std::{net::SocketAddr, sync::Arc};
use tokio::fs;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::{info, instrument, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const DEFAULT_DATABASE_PATH: &str = "./geo_ip/GeoLite2-Country.mmdb";
const DEFAULT_BITY_CONFIG_PATH: &str = "./bity_config.json";

#[derive(Debug)]
pub struct AppState {
    geoip_db: Reader<Vec<u8>>,
    bity_config: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    // Tracing init
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "nym_exchange=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let port: u16 = env::var("PORT")
        .expect("Environment variable PORT not set")
        .parse()
        .with_context(|| "Environment variable PORT invalid value")?;

    let bity_config_path = env::var("BITY_CONFIG_PATH").unwrap_or_else(|e| {
        warn!(
            "Env variable BITY_CONFIG_PATH is not set: {} - Fallback to {}",
            e, DEFAULT_BITY_CONFIG_PATH
        );
        DEFAULT_BITY_CONFIG_PATH.to_owned()
    });
    let bity_config = fs::read_to_string(bity_config_path).await?;

    let db_path = env::var("GEOIP_DB_PATH").unwrap_or_else(|e| {
        warn!(
            "Env variable GEOIP_DB_PATH is not set: {} - Fallback to {}",
            e, DEFAULT_DATABASE_PATH
        );
        DEFAULT_DATABASE_PATH.to_owned()
    });
    let reader = Reader::open_readfile(&db_path)
        .with_context(|| format!("Fail to open GeoLite2 database file {}", db_path))?;

    let state = Arc::new(AppState {
        geoip_db: reader,
        bity_config,
    });

    // Router setup
    let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));
    let serve_dir = get_service(serve_dir)
        .handle_error(handle_error)
        .route_layer(middleware::from_fn_with_state(state.clone(), ip_check));

    let app = Router::new()
        .route("/config.js", get(get_config))
        .route(
            "/403.html",
            get_service(ServeFile::new("assets/403.html")).handle_error(handle_error),
        )
        .nest_service("/assets", serve_dir.clone())
        .fallback_service(serve_dir)
        .layer(TraceLayer::new_for_http())
        .layer(Extension(state));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    info!("❱ listening on {} ❰", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

#[instrument(skip_all, level = "info")]
pub async fn get_config(state: Extension<Arc<AppState>>) -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "application/javascript")],
        format!(
            "window.bityConfiguration = {{ exchangeClient: JSON.parse(`{}`) }};",
            state.bity_config
        ),
    )
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
