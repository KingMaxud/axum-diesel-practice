use crate::domain::models::auth::AuthError;
use crate::handlers::auth::{get_client, LoginParams, UserData};
use crate::infra::repositories::{auth_repository, auth_repository::NewOauth2Record};
use crate::AppState;
use axum::{
    extract::{Host, Query, State},
    response::Redirect,
    Extension,
};
use oauth2::{CsrfToken, PkceCodeChallenge, Scope};

pub async fn login(
    Extension(user_data): Extension<Option<UserData>>,
    Query(params): Query<LoginParams>,
    State(state): State<AppState>,
    Host(hostname): Host,
) -> Result<Redirect, AuthError> {
    if user_data.is_some() {
        // check if already authenticated
        return Ok(Redirect::to("/"));
    }
    let client = get_client(hostname)?;

    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.email".to_string(),
        ))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    let new_record = NewOauth2Record {
        csrf_state: csrf_state.secret().to_owned(),
        pkce_code_verifier: String::from(pkce_code_verifier.secret()),
        return_url: params.return_url.unwrap_or_else(|| "/".to_string()),
    };

    auth_repository::insert_oauth2_record(&state.pool, new_record)
        .await
        .map_err(AuthError::InfraError)?;

    Ok(Redirect::to(authorize_url.as_str()))
}
