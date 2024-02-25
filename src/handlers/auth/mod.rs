use crate::domain::models::auth::AuthError;
use dotenvy::var;
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, RevocationUrl, TokenUrl,
};
use serde::Deserialize;
use uuid::Uuid;

pub mod login;
pub mod oauth_return;
pub mod profile;

#[derive(Deserialize)]
pub struct LoginParams {
    return_url: Option<String>,
}

#[derive(Clone, Debug)]
pub struct UserData {
    pub user_id: Uuid,
    pub user_email: String,
}

pub fn get_client(hostname: String) -> Result<BasicClient, AuthError> {
    let google_client_id = ClientId::new(var("GOOGLE_CLIENT_ID")?);
    let google_client_secret = ClientSecret::new(var("GOOGLE_CLIENT_SECRET")?);
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .map_err(|_| "OAuth: invalid authorization endpoint URL")?;
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
        .map_err(|_| "OAuth: invalid token endpoint URL")?;

    let protocol = if hostname.starts_with("localhost") || hostname.starts_with("127.0.0.1") {
        "http"
    } else {
        "https"
    };

    let redirect_url = format!("{}://{}/api/auth/oauth_return", protocol, hostname);

    // Set up the config for the Google OAuth2 process.
    let client = BasicClient::new(
        google_client_id,
        Some(google_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).map_err(|_| "OAuth: invalid redirect URL")?)
    .set_revocation_uri(
        RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
            .map_err(|_| "OAuth: invalid revocation endpoint URL")?,
    );
    Ok(client)
}
