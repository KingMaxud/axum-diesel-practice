use crate::config::config;
use crate::routes::app_router;
use deadpool_diesel::postgres::{Manager, Pool};
use tracing::log::debug;

mod config;
mod domain;
mod handlers;
mod infra;
mod middlewares;
mod routes;

#[derive(Clone)]
pub struct AppState {
    pool: Pool,
}

#[tokio::main]
async fn main() {
    let config = config().await;

    env_logger::init();

    let manager = Manager::new(
        config.db_url().to_string(),
        deadpool_diesel::Runtime::Tokio1,
    );
    let pool = Pool::builder(manager).build().unwrap();

    let state = AppState { pool };

    let app = app_router(state.clone()).with_state(state);

    let host = config.server_host();
    let port = config.server_port();

    let address = format!("{}:{}", host, port);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    debug!("LISTENING on {:?}\n", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
