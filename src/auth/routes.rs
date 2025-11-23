use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use derive_new::new;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    auth::logic,
    error::{ApiError, AppError},
    state::AppState,
    user::{self},
};

#[utoipa::path(
    post,
    path = "/auth/register",
    request_body(content = RegisterBody, content_type = "application/json"),
    responses(
        (status = 201, description = "Register OK"),
        (status = 400, description = "Invalid request", body = ApiError),
        (status = 409, description = "Username or email already taken"),
        (status = 500, description = "Internal server error"),
    ),
)]
pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<RegisterBody>,
) -> Result<RegisterResponse, AppError> {
    let username = body.username.parse()?;
    let email = body.email.parse()?;
    let password = body.password.parse()?;

    let mut tx = state.db_pool.begin().await?;

    let user_id = user::logic::create(&mut tx, username, email).await?;
    logic::create_entry(&mut tx, user_id, password).await?;

    tx.commit().await?;

    Ok(RegisterResponse::new(user_id.value()))
}

#[derive(Deserialize, ToSchema)]
pub struct RegisterBody {
    username: String,
    email: String,
    password: String,
}

#[derive(new, Serialize, ToSchema)]
pub struct RegisterResponse {
    user_id: Uuid,
}

impl IntoResponse for RegisterResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}
