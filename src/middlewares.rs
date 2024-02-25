use crate::domain::models::auth::AuthError;
use crate::handlers::auth::UserData;
use crate::infra::repositories::{user_repository, user_sessions_repository};
use crate::AppState;
use axum::{
    body::Body,
    extract::State,
    http::Request,
    middleware::Next,
    response::{IntoResponse, Redirect},
};
use axum_extra::TypedHeader;
use chrono::Utc;
use headers::Cookie;

pub async fn inject_user_data(
    State(state): State<AppState>,
    cookie: Option<TypedHeader<Cookie>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, AuthError> {
    if let Some(cookie) = cookie {
        if let Some(session_token) = cookie.get("session_token") {
            let session_token: Vec<&str> = session_token.split('_').collect();
            let user_session = user_sessions_repository::get_by_first_part_token(
                &state.pool,
                session_token[0].chars().collect::<String>(),
            )
            .await
            .map_err(AuthError::InfraError);

            if let Ok(query) = user_session {
                if let Ok(session_token_p2_db) = query.session_token_p2.as_bytes().try_into() {
                    if let Ok(session_token_p2_cookie) = session_token
                        .get(1)
                        .copied()
                        .unwrap_or_default()
                        .as_bytes()
                        .try_into()
                    {
                        if constant_time_eq::constant_time_eq_n::<36>(
                            session_token_p2_cookie,
                            session_token_p2_db,
                        ) {
                            let user_id = query.user_id;
                            let expires_at = query.expires_at;
                            if expires_at > Utc::now().timestamp() {
                                let user = user_repository::get(&state.pool, user_id)
                                    .await
                                    .map_err(AuthError::InfraError);
                                if let Ok(query) = user {
                                    let user_email = query.email;
                                    request.extensions_mut().insert(Some(UserData {
                                        user_id,
                                        user_email,
                                    }));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(next.run(request).await)
}

pub async fn check_auth(
    request: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, AuthError> {
    if request
        .extensions()
        .get::<Option<UserData>>()
        .ok_or("check_auth: extensions have no UserData")?
        .is_some()
    {
        Ok(next.run(request).await)
    } else {
        let login_url = "/api/auth/login?return_url=".to_owned() + &*request.uri().to_string();
        Ok(Redirect::to(login_url.as_str()).into_response())
    }
}
