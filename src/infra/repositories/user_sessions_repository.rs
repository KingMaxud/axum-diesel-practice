use crate::infra::db::schema::user_sessions;
use crate::infra::errors::{adapt_infra_error, InfraError};
use diesel::{Insertable, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use serde::{Deserialize, Serialize};
use tracing::debug;
use uuid::Uuid;

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = user_sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_token_p1: String,
    pub session_token_p2: String,
    pub created_at: i64,
    pub expires_at: i64,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = user_sessions)]
pub struct NewUserSession {
    pub user_id: Uuid,
    pub session_token_p1: String,
    pub session_token_p2: String,
    pub created_at: i64,
    pub expires_at: i64,
}

pub async fn insert(
    pool: &deadpool_diesel::postgres::Pool,
    new_user_session: NewUserSession,
) -> Result<(), InfraError> {
    debug!("->> {:<12} - insert", "INFRASTRUCTURE");

    // Get a database connection from the pool and handle any potential errors
    let conn = pool.get().await.map_err(adapt_infra_error)?;

    // Insert the new post into the 'posts' table returning the inserted post
    conn.interact(|conn| {
        diesel::insert_into(user_sessions::table)
            .values(new_user_session)
            .returning(UserSession::as_returning())
            .get_result(conn)
    })
    .await
    .map_err(adapt_infra_error)?
    .map_err(adapt_infra_error)?;

    Ok(())
}
