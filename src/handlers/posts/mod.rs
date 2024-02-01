use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod create_post;
pub mod get_post;
pub mod list_posts;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostResponse {
    id: Uuid,
    title: String,
    body: String,
    published: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    title: String,
    body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPostsResponse {
    posts: Vec<PostResponse>,
}
