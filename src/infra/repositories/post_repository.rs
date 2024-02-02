use crate::domain::models::post::PostModel;
use crate::handlers::posts::UpdatePostRequest;
use crate::infra::{
    db::schema::posts,
    errors::{adapt_infra_error, InfraError},
};
use diesel::{
    AsChangeset, ExpressionMethods, Insertable, PgTextExpressionMethods, QueryDsl, Queryable,
    RunQueryDsl, Selectable, SelectableHelper,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::pg::Pg))] // Check compatibility with PostgreSQL
pub struct PostDb {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = posts)] // Use the 'posts' table
pub struct NewPostDb {
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Deserialize)]
pub struct PostsFilter {
    published: Option<bool>,
    title_contains: Option<String>,
}

#[derive(AsChangeset)]
#[diesel(table_name = posts)]
struct UpdatePostChangeset {
    title: Option<String>,
    body: Option<String>,
    published: Option<bool>,
}

pub async fn insert(
    pool: &deadpool_diesel::postgres::Pool,
    new_post: NewPostDb,
) -> Result<PostModel, InfraError> {
    println!("->> {:<12} - insert", "INFRASTRUCTURE");

    // Get a database connection from the pool and handle any potential errors
    let conn = pool.get().await.map_err(adapt_infra_error)?;

    // Insert the new post into the 'posts' table returning the inserted post
    let res = conn
        .interact(|conn| {
            diesel::insert_into(posts::table)
                .values(new_post)
                .returning(PostDb::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    Ok(adapt_post_db_to_post(res))
}

pub async fn get(
    pool: &deadpool_diesel::postgres::Pool,
    id: Uuid,
) -> Result<PostModel, InfraError> {
    println!("->> {:<12} - get", "INFRASTRUCTURE");

    // Get a database connection from the pool and handle any potential errors
    let conn = pool.get().await.map_err(adapt_infra_error)?;

    // Query the 'posts' table to retrieve the post by its ID
    let res = conn
        .interact(move |conn| {
            posts::table
                .filter(posts::id.eq(id))
                .select(PostDb::as_select()) // Select the post
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    // Adapt the database representation to the application's domain model
    Ok(adapt_post_db_to_post(res))
}

pub async fn get_all(
    pool: &deadpool_diesel::postgres::Pool,
    filter: PostsFilter,
) -> Result<Vec<PostModel>, InfraError> {
    println!("->> {:<12} - get_all", "INFRASTRUCTURE");

    // Get a database connection from the pool and handle any potential errors
    let conn = pool.get().await.map_err(adapt_infra_error)?;

    // Build a dynamic query for retrieving posts
    let res = conn
        .interact(move |conn| {
            let mut query = posts::table.into_boxed::<diesel::pg::Pg>();

            // Apply filtering conditions if provided
            if let Some(published) = filter.published {
                query = query.filter(posts::published.eq(published));
            }

            if let Some(title_contains) = filter.title_contains {
                query = query.filter(posts::title.ilike(format!("%{}%", title_contains)));
            }

            // Select the posts matching the query
            query.select(PostDb::as_select()).load::<PostDb>(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    // Adapt the database representations to the application's domain models
    let posts: Vec<PostModel> = res
        .into_iter()
        .map(|post_db| adapt_post_db_to_post(post_db))
        .collect();

    Ok(posts)
}

pub async fn update(
    pool: &deadpool_diesel::postgres::Pool,
    id: Uuid,
    updated_post: UpdatePostRequest,
) -> Result<PostModel, InfraError> {
    println!("->> {:<12} - update", "INFRASTRUCTURE");

    // Get a database connection from the pool and handle any potential errors
    let conn = pool.get().await.map_err(adapt_infra_error)?;

    let changeset = UpdatePostChangeset {
        title: updated_post.title,
        body: updated_post.body,
        published: updated_post.published,
    };

    let res = conn
        .interact(move |conn| {
            diesel::update(posts::table.filter(posts::id.eq(id)))
                .set(&changeset)
                .returning(PostDb::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    Ok(adapt_post_db_to_post(res))
}

pub async fn delete(
    pool: &deadpool_diesel::postgres::Pool,
    id: Uuid,
) -> Result<PostModel, InfraError> {
    println!("->> {:<12} - delete", "INFRASTRUCTURE");

    // Get a database connection from the pool and handle any potential errors
    let conn = pool.get().await.map_err(adapt_infra_error)?;

    let res = conn
        .interact(move |conn| {
            diesel::delete(posts::table.filter(posts::id.eq(id)))
                .returning(PostDb::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    Ok(adapt_post_db_to_post(res))
}

fn adapt_post_db_to_post(post_db: PostDb) -> PostModel {
    PostModel {
        id: post_db.id,
        title: post_db.title,
        body: post_db.body,
        published: post_db.published,
    }
}
