use utoipa::OpenApi;

use crate::api::routes::health;

#[derive(OpenApi)]
#[openapi(
    paths(
        health::healthz
    ),
    components(
        schemas(health::HealthResponse)
    ),
    tags(
        (name = "health", description = "Health endpoints")
    )
)]
pub struct ApiDoc;