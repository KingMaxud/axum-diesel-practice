use crate::infra::db::schema::oauth2_records;
use crate::infra::errors::{adapt_infra_error, InfraError};
use diesel::{
    ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper,
};
use serde::{Deserialize, Serialize};
use tracing::log::debug;
use uuid::Uuid;

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = oauth2_records)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Oauth2Record {
    pub id: Uuid,
    pub csrf_state: String,
    pub pkce_code_verifier: String,
    pub return_url: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = oauth2_records)]
pub struct NewOauth2Record {
    pub csrf_state: String,
    pub pkce_code_verifier: String,
    pub return_url: String,
}

pub async fn insert_oauth2_record(
    pool: &deadpool_diesel::postgres::Pool,
    new_record: NewOauth2Record,
) -> Result<(), InfraError> {
    debug!("->> {:<12} - insert_oauth2_record", "INFRASTRUCTURE");

    // Get a database connection from the pool and handle any potential errors
    let conn = pool.get().await.map_err(adapt_infra_error)?;

    conn.interact(|conn| {
        diesel::insert_into(oauth2_records::table)
            .values(new_record)
            .returning(Oauth2Record::as_returning())
            .get_result(conn)
    })
    .await
    .map_err(adapt_infra_error)?
    .map_err(adapt_infra_error)?;

    Ok(())
}

pub async fn delete_oauth2_record(
    pool: &deadpool_diesel::postgres::Pool,
    csrf_state: String,
) -> Result<(String, String), InfraError> {
    debug!("->> {:<12} - delete_oauth2_record", "INFRASTRUCTURE");

    // Get a database connection from the pool and handle any potential errors
    let conn = pool.get().await.map_err(adapt_infra_error)?;

    let res = conn
        .interact(move |conn| {
            diesel::delete(oauth2_records::table.filter(oauth2_records::csrf_state.eq(csrf_state)))
                .returning(Oauth2Record::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    Ok((res.pkce_code_verifier, res.return_url))
}
