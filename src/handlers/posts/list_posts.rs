// Import necessary modules and types
use axum::extract::{Query, State};
use axum::Json;
use tracing::log::debug;

// Import internal modules and types
use crate::domain::models::post::{PostError, PostModel};
use crate::handlers::posts::{ListPostsResponse, PostResponse};
use crate::infra::repositories::post_repository::{get_all, PostsFilter};
use crate::AppState;

// Define the handler function for listing posts with optional query parameters
pub async fn list_posts(
    State(state): State<AppState>,
    Query(params): Query<PostsFilter>,
) -> Result<Json<ListPostsResponse>, PostError> {
    debug!("->> {:<12} - list_posts", "HANDLER");

    let posts = get_all(&state.pool, params)
        .await
        .map_err(|_| PostError::InternalServerError)?;

    // Convert the retrieved list of PostModel instances to a ListPostsResponse
    Ok(Json(adapt_posts_to_list_posts_response(posts)))
}

// Helper function to adapt a single PostModel to a PostResponse
fn adapt_post_to_post_response(post: PostModel) -> PostResponse {
    PostResponse {
        id: post.id,
        title: post.title,
        body: post.body,
        published: post.published,
    }
}

// Helper function to adapt a list of PostModel instances to a ListPostsResponse
fn adapt_posts_to_list_posts_response(posts: Vec<PostModel>) -> ListPostsResponse {
    // Map each PostModel to a PostResponse and collect them into a Vec<PostResponse>
    let posts_response: Vec<PostResponse> =
        posts.into_iter().map(adapt_post_to_post_response).collect();

    // Create a ListPostsResponse containing the list of PostResponses
    ListPostsResponse {
        posts: posts_response,
    }
}
