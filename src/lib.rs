pub mod api;
pub mod config;

use axum::Router;
use tracing_subscriber::EnvFilter;

pub fn build_app() -> Router {
    api::router::build_router()
}

pub fn init_tracing(log_level: &str) {
    // Use provided log_level if RUST_LOG isn't set
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(log_level));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();
}