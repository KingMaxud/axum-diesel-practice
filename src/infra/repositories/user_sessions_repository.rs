use crate::domain::models::user_session::UserSessionModel;
use crate::infra::db::schema::user_sessions;
use crate::infra::errors::{adapt_infra_error, InfraError};
use diesel::{
    ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper,
};
use serde::{Deserialize, Serialize};
use tracing::debug;
use uuid::Uuid;

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = user_sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserSessionDb {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_token_p1: String,
    pub session_token_p2: String,
    pub created_at: i64,
    pub expires_at: i64,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = user_sessions)]
pub struct NewUserSessionDb {
    pub user_id: Uuid,
    pub session_token_p1: String,
    pub session_token_p2: String,
    pub created_at: i64,
    pub expires_at: i64,
}

pub async fn insert(
    pool: &deadpool_diesel::postgres::Pool,
    new_user_session: NewUserSessionDb,
) -> Result<(), InfraError> {
    debug!("->> {:<12} - insert", "INFRASTRUCTURE");

    // Get a database connection from the pool and handle any potential errors
    let conn = pool.get().await.map_err(adapt_infra_error)?;

    // Insert the new post into the 'posts' table returning the inserted post
    conn.interact(|conn| {
        diesel::insert_into(user_sessions::table)
            .values(new_user_session)
            .returning(UserSessionDb::as_returning())
            .get_result(conn)
    })
    .await
    .map_err(adapt_infra_error)?
    .map_err(adapt_infra_error)?;

    Ok(())
}

pub async fn get_by_first_part_token(
    pool: &deadpool_diesel::postgres::Pool,
    session_token_p1: String,
) -> Result<UserSessionModel, InfraError> {
    debug!("->> {:<12} - get", "INFRASTRUCTURE");

    // Get a database connection from the pool and handle any potential errors
    let conn = pool.get().await.map_err(adapt_infra_error)?;

    let res = conn
        .interact(|conn| {
            user_sessions::table
                .filter(user_sessions::session_token_p1.eq(session_token_p1))
                .select(UserSessionDb::as_select())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    Ok(adapt_user_session_to_user_session_model(res))
}

fn adapt_user_session_to_user_session_model(user_session: UserSessionDb) -> UserSessionModel {
    UserSessionModel {
        id: user_session.id,
        user_id: user_session.user_id,
        session_token_p1: user_session.session_token_p1,
        session_token_p2: user_session.session_token_p2,
        created_at: user_session.created_at,
        expires_at: user_session.expires_at,
    }
}
