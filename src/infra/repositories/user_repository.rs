use crate::domain::models::user::UserModel;
use crate::infra::db::schema::users;
use crate::infra::errors::{adapt_infra_error, InfraError};
use diesel::{
    ExpressionMethods, Insertable, OptionalExtension, QueryDsl, Queryable, RunQueryDsl, Selectable,
    SelectableHelper,
};
use serde::{Deserialize, Serialize};
use tracing::log::debug;
use uuid::Uuid;

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserDb {
    pub id: Uuid,
    pub email: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUserDb {
    pub email: String,
}

pub async fn insert_if_not_exists(
    pool: &deadpool_diesel::postgres::Pool,
    email: String,
) -> Result<Uuid, InfraError> {
    debug!("->> {:<12} - insert_if_not_exists", "INFRASTRUCTURE");

    // Get a database connection from the pool and handle any potential errors
    let conn = pool.get().await.map_err(adapt_infra_error)?;

    let email_for_check = email.clone();
    // Check if user exists
    let existing_user = conn
        .interact(|conn| {
            users::table
                .filter(users::email.eq(email_for_check))
                .first::<UserDb>(conn)
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
                    let new_user = NewUserDb { email }; // Create a new user struct
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

pub async fn get(
    pool: &deadpool_diesel::postgres::Pool,
    id: Uuid,
) -> Result<UserModel, InfraError> {
    debug!("->> {:<12} - get", "INFRASTRUCTURE");

    // Get a database connection from the pool and handle any potential errors
    let conn = pool.get().await.map_err(adapt_infra_error)?;

    let res = conn
        .interact(move |conn| {
            users::table
                .filter(users::id.eq(id))
                .select(UserDb::as_select())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    Ok(adapt_user_db_to_user_model(res))
}

fn adapt_user_db_to_user_model(user_db: UserDb) -> UserModel {
    UserModel {
        id: user_db.id,
        email: user_db.email,
    }
}
