use axum::{routing::get, Router};
use axum::http::{header::HeaderName, Request};
use tower::ServiceBuilder;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use super::openapi::ApiDoc;

pub fn build_router() -> Router {
    let request_id_header = HeaderName::from_static("x-request-id");

    let middleware = ServiceBuilder::new()
        .layer(SetRequestIdLayer::new(
            request_id_header.clone(),
            MakeRequestUuid,
        ))
        .layer(PropagateRequestIdLayer::new(request_id_header.clone()))
        .layer(
            TraceLayer::new_for_http().make_span_with(move |req: &Request<_>| {
                let request_id = req
                    .headers()
                    .get(&request_id_header)
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("-");

                tracing::info_span!(
                    "http_request",
                    method = %req.method(),
                    uri = %req.uri(),
                    request_id = %request_id,
                )
            }),
        );

    Router::new()
        .merge(health_routes())
        .merge(SwaggerUi::new("/docs").url("/openapi.json", ApiDoc::openapi()))
        .layer(middleware)
}

fn health_routes() -> Router {
    Router::new().route("/healthz", get(super::routes::health::healthz))
}