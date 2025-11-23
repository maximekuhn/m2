use axum::{Json, http::StatusCode, response::IntoResponse};
use derive_new::new;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("invalid input: {field}{reason}", reason = reason.map(|r| format!(": {r}")).unwrap_or("".to_string()))]
    InvalidInput {
        field: &'static str,
        reason: Option<&'static str>,
    },

    #[error("email already taken")]
    EmailAlreadyTaken,

    #[error("username already taken")]
    UsernameAlreadyTaken,

    #[error("password hash error: {0}")]
    PasswordHash(argon2::password_hash::Error),

    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(value: argon2::password_hash::Error) -> Self {
        Self::PasswordHash(value)
    }
}

impl From<email_address::Error> for AppError {
    fn from(_: email_address::Error) -> Self {
        Self::InvalidInput {
            field: "email",
            reason: None,
        }
    }
}

#[derive(new, Serialize, ToSchema)]
pub struct ApiError {
    detail: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status_code = match &self {
            AppError::InvalidInput { .. } => StatusCode::BAD_REQUEST,
            AppError::EmailAlreadyTaken => StatusCode::CONFLICT,
            AppError::UsernameAlreadyTaken => StatusCode::CONFLICT,
            AppError::PasswordHash(..) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Database(..) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let error = match &self {
            AppError::InvalidInput { .. } => Some(ApiError::new(self.to_string())),
            AppError::EmailAlreadyTaken => Some(ApiError::new(self.to_string())),
            AppError::UsernameAlreadyTaken => Some(ApiError::new(self.to_string())),
            AppError::PasswordHash(..) => None,
            AppError::Database(..) => None,
        };

        match error {
            Some(error) => (status_code, Json(error)).into_response(),
            None => status_code.into_response(),
        }
    }
}
