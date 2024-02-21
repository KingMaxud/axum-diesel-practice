use crate::infra::errors::{Error, InfraError};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

#[derive(Debug)]
pub enum AuthError {
    EmailAddressIsNotVerified,
    InfraError(InfraError),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            Self::InfraError(db_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {}", db_error),
            ),
            Self::EmailAddressIsNotVerified => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "OAuth: email address is not verified".to_string(),
            ),
        };

        (
            status,
            Json(
                json!({"resource":"Auth", "message": err_msg, "happened_at" : chrono::Utc::now() }),
            ),
        )
            .into_response()
    }
}

impl From<dotenvy::Error> for AuthError {
    fn from(err: dotenvy::Error) -> Self {
        Self::InfraError(err.as_infra_error())
    }
}

impl From<&str> for AuthError {
    fn from(err: &str) -> Self {
        Self::InfraError(err.as_infra_error())
    }
}
