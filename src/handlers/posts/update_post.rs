use crate::domain::models::post::PostError;
use crate::handlers::posts::{PostResponse, UpdatePostRequest};
use crate::infra::repositories::post_repository;
use crate::AppState;
use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;

pub async fn update_post(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(updated_post): Json<UpdatePostRequest>,
) -> Result<Json<PostResponse>, PostError> {
    println!("->> {:<12} - update_post", "HANDLER");

    let updated_response = post_repository::update(&state.pool, id, updated_post)
        .await
        .map_err(PostError::InfraError)?;

    // Create a PostResponse instance from the newly updated post
    let post_response = PostResponse {
        id: updated_response.id,
        title: updated_response.title,
        body: updated_response.body,
        published: updated_response.published,
    };

    // Return the response as JSON with a success status
    Ok(Json(post_response))
}
