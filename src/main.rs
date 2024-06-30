mod db;
mod routes;
mod schema;
mod utils;

use axum::{
    routing::{get, post},
    Router,
};
use db::init_db;
use routes::ingredient_request::{create_ingredient_request, list_ingredient_requests};
use routes::user::{register, sign_in};
use routes::{
    ingredient::{create_ingredient, list_ingredients},
    recipe::{create_recipe, get_recipe, get_recipes_by_user_id, list_recipes},
};
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
        .route("/register", post(register))
        .route("/login", get(sign_in))
        .route("/ingredient", post(create_ingredient))
        .route("/ingredients", get(list_ingredients))
        .route("/ingredient-request", post(create_ingredient_request))
        .route("/ingredient-requests", get(list_ingredient_requests))
        .route("/recipe", post(create_recipe))
        .route("/recipes", get(list_recipes))
        .route("/recipe/:id", get(get_recipe))
        .route("/recipes-from/:id", get(get_recipes_by_user_id))
        .with_state(db_pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
