use crate::domain::models::post::PostError;
use crate::handlers::posts::{CreatePostRequest, PostResponse};
use crate::infra::repositories::post_repository;
use crate::AppState;
use axum::{extract::State, Json};
use tracing::log::debug;

pub async fn create_post(
    State(state): State<AppState>,
    Json(new_post): Json<CreatePostRequest>,
) -> Result<Json<PostResponse>, PostError> {
    debug!("->> {:<12} - create_post", "HANDLER");

    let new_post_db = post_repository::NewPostDb {
        title: new_post.title,
        body: new_post.body,
        published: false,
    };

    // Insert the new post into the database using the repository
    let created_post = post_repository::insert(&state.pool, new_post_db)
        .await
        .map_err(PostError::InfraError)?;

    // Create a PostResponse instance from the newly created post
    let post_response = PostResponse {
        id: created_post.id,
        title: created_post.title,
        body: created_post.body,
        published: created_post.published,
    };

    // Return the response as JSON with a success status
    Ok(Json(post_response))
}
