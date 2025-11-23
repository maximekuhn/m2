use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(crate::auth::routes::register))]
pub struct ApiDoc;
