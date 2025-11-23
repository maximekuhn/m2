use axum::{Router, routing::post};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{auth, openapi::ApiDoc, state::AppState};

pub fn app_router(state: AppState, enable_swagger: bool) -> Router {
    let auth_router = Router::new()
        .route("/register", post(auth::routes::register))
        .with_state(state);

    let mut router = Router::new();

    if enable_swagger {
        router = Router::new()
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));
    }

    router.nest("/auth", auth_router)
}
