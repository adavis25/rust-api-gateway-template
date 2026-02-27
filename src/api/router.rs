use axum::{routing::get, Router};
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;

use super::openapi::ApiDoc;

pub fn build_router() -> Router {
    Router::new()
        .merge(health_routes())
        .merge(
            SwaggerUi::new("/docs")
                .url("/openapi.json", ApiDoc::openapi()),
        )
}

fn health_routes() -> Router {
    Router::new().route("/healthz", get(super::routes::health::healthz))
}