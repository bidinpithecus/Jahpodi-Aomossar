mod db;
mod routes;
mod schema;
mod utils;

use axum::{
    routing::{get, post},
    Router,
};
use db::init_db;
use routes::user::{create_user, list_users};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_pool = init_db().await;

    let app = Router::new()
        .route("/user/list", get(list_users))
        .route("/user/create", post(create_user))
        .with_state(db_pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
