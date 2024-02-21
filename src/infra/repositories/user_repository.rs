use crate::infra::db::schema::users;
use crate::infra::errors::{adapt_infra_error, InfraError};
use diesel::{
    ExpressionMethods, Insertable, OptionalExtension, QueryDsl, Queryable, RunQueryDsl, Selectable,
};
use serde::{Deserialize, Serialize};
use tracing::log::debug;
use uuid::Uuid;

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub email: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
}

pub async fn insert_new_user_if_not_exists(
    pool: &deadpool_diesel::postgres::Pool,
    email: String,
) -> Result<Uuid, InfraError> {
    debug!(
        "->> {:<12} - insert_new_user_if_not_exists",
        "INFRASTRUCTURE"
    );

    // Get a database connection from the pool and handle any potential errors
    let conn = pool.get().await.map_err(adapt_infra_error)?;

    let email_for_check = email.clone();
    // Check if user exists
    let existing_user = conn
        .interact(|conn| {
            users::table
                .filter(users::email.eq(email_for_check))
                .first::<User>(conn)
                .optional()
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    // Create new user if necessary
    let user_id = match existing_user {
        Some(user) => user.id, // User already exists, use their ID
        None => {
            let id = conn
                .interact(|conn| {
                    let new_user = NewUser { email }; // Create a new user struct
                    diesel::insert_into(users::table)
                        .values(&new_user)
                        .returning(users::id)
                        .get_result(conn)
                })
                .await
                .map_err(adapt_infra_error)?
                .map_err(adapt_infra_error)?;

            id
        }
    };

    Ok(user_id)
}
