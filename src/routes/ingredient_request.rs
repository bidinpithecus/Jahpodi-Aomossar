use crate::db::ingredient_request::{IngredientRequest, NewIngredientRequest};
use crate::schema::ingredient_request;
use crate::utils::internal_error;

use axum::{
    extract::State,
    http::StatusCode,
    response::{Json, Html},
};
use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;

pub async fn create_ingredient_request(
    State(pool): State<Pool>,
    Json(new_ingredient_request): Json<NewIngredientRequest>,
) -> Result<Json<IngredientRequest>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| {
            diesel::insert_into(ingredient_request::table)
                .values(new_ingredient_request)
                .returning(IngredientRequest::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

pub async fn list_ingredient_requests(
    State(pool): State<Pool>,
) -> Result<Json<Vec<IngredientRequest>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| ingredient_request::table.select(IngredientRequest::as_select()).load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

pub async fn show_ingredient_page() -> Html<String> {
    let html_content = tokio::fs::read_to_string("static/ingredient.html").await.unwrap();
    Html(html_content)
}
