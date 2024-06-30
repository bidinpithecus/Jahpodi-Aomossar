use crate::db::ingredient::{Ingredient, NewIngredient};
use crate::schema::ingredient;
use crate::utils::internal_error;
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;

pub async fn create_ingredient(
    State(pool): State<Pool>,
    Json(new_ingredient): Json<NewIngredient>,
) -> Result<Json<Ingredient>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| {
            diesel::insert_into(ingredient::table)
                .values(new_ingredient)
                .returning(Ingredient::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

pub async fn list_ingredients(
    State(pool): State<Pool>,
) -> Result<Json<Vec<Ingredient>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| ingredient::table.select(Ingredient::as_select()).load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}
