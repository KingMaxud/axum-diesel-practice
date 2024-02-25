use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct UserSessionModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_token_p1: String,
    pub session_token_p2: String,
    pub created_at: i64,
    pub expires_at: i64,
}
