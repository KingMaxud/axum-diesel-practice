use crate::handlers::auth::login::login;
use crate::handlers::auth::oauth_return::oauth_return;
use crate::handlers::auth::profile::profile;
use crate::handlers::auth::UserData;
use crate::handlers::posts::create_post::create_post;
use crate::handlers::posts::delete_post::delete_post;
use crate::handlers::posts::get_post::get_post;
use crate::handlers::posts::list_posts::list_posts;
use crate::handlers::posts::update_post::update_post;
use crate::middlewares::{check_auth, inject_user_data};
use crate::AppState;
use axum::{
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Extension, Router,
};
use tracing::log::debug;

pub fn app_router(state: AppState) -> Router<AppState> {
    let user_data: Option<UserData> = None;

    Router::new()
        .route("/", get(root))
        .nest("/api/post", post_routes(state.clone()))
        .nest("/api/auth", auth_routes(state.clone()))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            inject_user_data,
        ))
        .layer(Extension(user_data))
        .fallback(handler_404)
}

// Handler for the root path "/"
async fn root() -> &'static str {
    "Server is running!" // Return a simple message indicating the server is running
}

async fn handler_404() -> impl IntoResponse {
    debug!("->> {:<12} - handler_404", "HANDLER");

    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}

fn post_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(create_post))
        .route("/:id", get(get_post))
        .route("/:id", patch(update_post))
        .route("/:id", delete(delete_post))
        .route("/", get(list_posts))
        .with_state(state)
}
fn auth_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/profile", get(profile))
        .route_layer(middleware::from_fn_with_state(state.clone(), check_auth))
        .route("/login", get(login))
        .route("/oauth_return", get(oauth_return))
        .with_state(state)
}
