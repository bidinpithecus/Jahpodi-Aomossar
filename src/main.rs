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
    answer::get_answers_by_question_id, home::show_home_page, recipe::{create_recipe, show_recipe_page}, user::{register, show_login_page, show_recipes_page, show_register_page, sign_in}
};
use routes::{
    ingredient::{create_ingredient, list_ingredients},
    recipe::{show_post_recipe_page, get_recipe, get_recipes_by_user_id, list_recipes},
};
use routes::{
    ingredient_request::{show_ingredient_page, create_ingredient_request, list_ingredient_requests},
    question::ask_question,
};
use routes::question::get_questions_by_recipe_id;
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {
    let db_pool = init_db().await;

    let app = Router::new()
        .route("/", get(show_home_page))
        .route("/home", get(show_home_page))
        .route("/api/register", post(register))
        .route("/register", get(show_register_page))
        .route("/api/login", post(sign_in))
        .route("/login", get(show_login_page))
        .route("/recipes", get(show_recipes_page))
        .route("/api/recipes", get(list_recipes))
        .route("/post_recipe", get(show_post_recipe_page))
        .route(
            "/api/ingredient",
            post(create_ingredient),
        )
        .route("/api/ingredients", get(list_ingredients))
        .route(
            "/ingredient-request",
            post(create_ingredient_request),
        )
        .route("/ingredient-requests", get(list_ingredient_requests))
        .route("/ingredient", get(show_ingredient_page))
        .route(
            "/api/recipe",
            post(create_recipe),
        )
        .route("/api/recipe/:id", get(get_recipe))
        .route("/recipe/:id", get(show_recipe_page))
        .route("/api/full-recipe/:id", get(get_full_recipe))
        .route("/recipes-from/:id", get(get_recipes_by_user_id))
        .route(
            "/question",
            post(ask_question),
        )
        .route("/questions", get(get_questions_by_recipe_id))
        .route(
            "/answer",
            post(answer_question),
        )
        .route("/answers-for/:id", get(get_answers_by_question_id))
        .with_state(db_pool)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
