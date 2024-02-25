use crate::domain::models::auth::AuthError;
use crate::handlers::auth::UserData;
use axum::Extension;

pub async fn profile(
    Extension(user_data): Extension<Option<UserData>>,
) -> Result<String, AuthError> {
    match user_data {
        Some(user_data) => Ok(user_data.user_email),
        None => Err(AuthError::Unauthenticated),
    }
}
