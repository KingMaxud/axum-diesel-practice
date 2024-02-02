use crate::handlers::posts::create_post::create_post;
use crate::handlers::posts::delete_post::delete_post;
use crate::handlers::posts::get_post::get_post;
use crate::handlers::posts::list_posts::list_posts;
use crate::handlers::posts::update_post::update_post;
use crate::AppState;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Router,
};

pub fn app_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .nest("/api/post", post_routes(state.clone()))
        .fallback(handler_404)
}

// Handler for the root path "/"
async fn root() -> &'static str {
    "Server is running!" // Return a simple message indicating the server is running
}

async fn handler_404() -> impl IntoResponse {
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
