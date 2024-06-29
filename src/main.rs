mod db;
mod routes;
mod schema;
mod utils;

use axum::{
    routing::{get, post},
    Router,
};
use db::init_db;
use routes::ingredient::{create_ingredient, get_ingredient, list_ingredients};
use routes::ingredient_request::{create_ingredient_request, get_ingredient_request, list_ingredient_requests};
use routes::user::{create_user, get_user, list_users};
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
        .route("/user", get(list_users))
        .route("/user/:id", get(get_user))
        .route("/user", post(create_user))
        .route("/ingredient", get(list_ingredients))
        .route("/ingredient/:id", get(get_ingredient))
        .route("/ingredient", post(create_ingredient))
        .route("/ingredient-request", get(list_ingredient_requests))
        .route("/ingredient-request/:id", get(get_ingredient_request))
        .route("/ingredient-request", post(create_ingredient_request))
        .with_state(db_pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
