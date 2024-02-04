use crate::domain::models::post::PostError;
use crate::handlers::posts::PostResponse;
use crate::infra::repositories::post_repository;
use crate::AppState;
use axum::extract::{Path, State};
use axum::Json;
use tracing::log::debug;
use uuid::Uuid;

pub async fn delete_post(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<PostResponse>, PostError> {
    debug!("->> {:<12} - delete_post", "HANDLER");

    let deleted_response = post_repository::delete(&state.pool, id)
        .await
        .map_err(PostError::InfraError)?;

    // Create a PostResponse instance from the deleted post
    let post_response = PostResponse {
        id: deleted_response.id,
        title: deleted_response.title,
        body: deleted_response.body,
        published: deleted_response.published,
    };

    // Return the response as JSON with a success status
    Ok(Json(post_response))
}
