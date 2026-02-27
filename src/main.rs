use rust_api_gateway_template::{build_app, config::Settings};

#[tokio::main]
async fn main() {
    let settings = Settings::from_env();
    rust_api_gateway_template::init_tracing(&settings.log_level);

    let listener = tokio::net::TcpListener::bind(settings.server_addr)
        .await
        .expect("Failed to bind address");

    tracing::info!("Listening on {}", settings.server_addr);

    axum::serve(listener, build_app()).await.unwrap();
}