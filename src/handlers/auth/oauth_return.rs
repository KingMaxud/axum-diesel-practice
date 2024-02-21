use crate::domain::models::auth::AuthError;
use crate::handlers::auth::get_client;
use crate::infra::repositories::{auth_repository, user_repository, user_sessions_repository};
use crate::AppState;
use axum::{
    extract::{Host, Query, State},
    response::{IntoResponse, Redirect},
};
use chrono::Utc;
use oauth2::{reqwest::http_client, AuthorizationCode, CsrfToken, PkceCodeVerifier, TokenResponse};
use std::collections::HashMap;
use uuid::Uuid;

pub async fn oauth_return(
    Query(mut params): Query<HashMap<String, String>>,
    State(state): State<AppState>,
    Host(hostname): Host,
) -> Result<impl IntoResponse, AuthError> {
    let state_token = CsrfToken::new(params.remove("state").ok_or("OAuth: without state")?);
    let code = AuthorizationCode::new(params.remove("code").ok_or("OAuth: without code")?);

    let (pkce_code_verifier, return_url) =
        auth_repository::delete_oauth2_record(&state.pool, state_token.secret().to_owned())
            .await
            .map_err(AuthError::InfraError)?;

    let pkce_code_verifier = PkceCodeVerifier::new(pkce_code_verifier);

    let client = get_client(hostname)?;
    let token_response = tokio::task::spawn_blocking(move || {
        client
            .exchange_code(code)
            .set_pkce_verifier(pkce_code_verifier)
            .request(http_client)
    })
    .await
    .map_err(|_| "OAuth: exchange_code failure")?
    .map_err(|_| "OAuth: tokio spawn blocking failure")?;
    let access_token = token_response.access_token().secret();

    // Get user info from Google
    let url =
        "https://www.googleapis.com/oauth2/v2/userinfo?oauth_token=".to_owned() + access_token;
    let body = reqwest::get(url)
        .await
        .map_err(|_| "OAuth: reqwest failed to query userinfo")?
        .text()
        .await
        .map_err(|_| "OAuth: reqwest received invalid userinfo")?;
    let mut body: serde_json::Value =
        serde_json::from_str(body.as_str()).map_err(|_| "OAuth: Serde failed to parse userinfo")?;
    let email = body["email"]
        .take()
        .as_str()
        .ok_or("OAuth: Serde failed to parse email address")?
        .to_owned();
    let verified_email = body["verified_email"]
        .take()
        .as_bool()
        .ok_or("OAuth: Serde failed to parse verified_email")?;
    if !verified_email {
        return Err(AuthError::EmailAddressIsNotVerified);
    }

    let user_id = user_repository::insert_new_user_if_not_exists(&state.pool, email)
        .await
        .map_err(AuthError::InfraError)?;

    let session_token_p1 = Uuid::new_v4().to_string();
    let session_token_p2 = Uuid::new_v4().to_string();
    let session_token = [session_token_p1.as_str(), "_", session_token_p2.as_str()].concat();
    let headers = axum::response::AppendHeaders([(
        axum::http::header::SET_COOKIE,
        "session_token=".to_owned()
            + &*session_token
            + "; path=/; httponly; secure; samesite=strict",
    )]);
    let now = Utc::now().timestamp();

    let new_user_session = user_sessions_repository::NewUserSession {
        user_id,
        session_token_p1,
        session_token_p2,
        created_at: now,
        expires_at: now + 60 * 60 * 24,
    };

    user_sessions_repository::insert(&state.pool, new_user_session)
        .await
        .map_err(AuthError::InfraError)?;

    Ok((headers, Redirect::to(return_url.as_str())))
}
