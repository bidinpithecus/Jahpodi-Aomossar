mod db;
mod routes;
mod schema;
mod utils;
use crate::routes::answer::answer_question;
use crate::routes::recipe::get_full_recipe;
use axum::{
    routing::{get, post},
    Router,
};
use db::init_db;
use routes::{
    answer::get_answers_by_question_id,
    user::{register, sign_in, show_register_page, show_login_page},
};
use routes::{
    ingredient::{create_ingredient, list_ingredients},
    recipe::{create_recipe, get_recipe, get_recipes_by_user_id, list_recipes},
};
use routes::{
    ingredient_request::{create_ingredient_request, list_ingredient_requests},
    question::ask_question,
};
use routes::{protected_routes::require_auth, question::get_questions_by_recipe_id};
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {
    let db_pool = init_db().await;

    let app = Router::new()
        .route("/api/register", post(register))
        .route("/register", get(show_register_page))
        .route("/api/login", post(sign_in))
        .route("/login", get(show_login_page))
        .route(
            "/ingredient",
            post(create_ingredient).layer(axum::middleware::from_fn(require_auth)),
        )
        .route("/ingredients", get(list_ingredients))
        .route(
            "/ingredient-request",
            post(create_ingredient_request).layer(axum::middleware::from_fn(require_auth)),
        )
        .route("/ingredient-requests", get(list_ingredient_requests))
        .route(
            "/recipe",
            post(create_recipe).layer(axum::middleware::from_fn(require_auth)),
        )
        .route("/recipes", get(list_recipes))
        .route("/recipe/:id", get(get_recipe))
        .route("/full-recipe/:id", get(get_full_recipe))
        .route("/recipes-from/:id", get(get_recipes_by_user_id))
        .route(
            "/question",
            post(ask_question).layer(axum::middleware::from_fn(require_auth)),
        )
        .route("/questions", get(get_questions_by_recipe_id))
        .route(
            "/answer",
            post(answer_question).layer(axum::middleware::from_fn(require_auth)),
        )
        .route("/answers-for/:id", get(get_answers_by_question_id))
        .with_state(db_pool)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
