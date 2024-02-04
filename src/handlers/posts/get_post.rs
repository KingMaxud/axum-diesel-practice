use crate::domain::models::post::{PostError, PostModel};
use crate::handlers::posts::PostResponse;
use crate::infra::errors::InfraError;
use crate::infra::repositories::post_repository;
use crate::AppState;
use axum::{
    extract::{Path, State},
    Json,
};
use tracing::log::debug;
use uuid::Uuid;

pub async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<PostResponse>, PostError> {
    debug!("->> {:<12} - get_post", "HANDLER");

    let post = post_repository::get(&state.pool, id)
        .await
        .map_err(|db_error| match db_error {
            InfraError::InternalServerError => PostError::InternalServerError,
            InfraError::NotFound => PostError::NotFound(id),
        })?;

    Ok(Json(adapt_post_to_post_response(post)))
}

fn adapt_post_to_post_response(post: PostModel) -> PostResponse {
    PostResponse {
        id: post.id,
        title: post.title,
        body: post.body,
        published: post.published,
    }
}
