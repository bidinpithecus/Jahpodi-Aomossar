use crate::db::recipe::{NewRecipe, Recipe};
use crate::schema::{user, recipe};
use crate::utils::internal_error;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;

pub async fn create_recipe(
    State(pool): State<Pool>,
    Json(new_recipe): Json<NewRecipe>,
) -> Result<Json<Recipe>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| {
            diesel::insert_into(recipe::table)
                .values(new_recipe)
                .returning(Recipe::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

pub async fn list_recipes(State(pool): State<Pool>) -> Result<Json<Vec<Recipe>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| recipe::table.select(Recipe::as_select()).load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

pub async fn get_recipes_by_user_id(
    State(pool): State<Pool>,
    Path(user_id): Path<i32>
) -> Result<Json<Vec<Recipe>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let user_exists: bool = conn
        .interact(move |conn| {
            diesel::select(diesel::dsl::exists(
                user::table.filter(user::id.eq(user_id))
            ))
            .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    if !user_exists {
        return Err((StatusCode::NOT_FOUND, "User not found".to_string()));
    }

    let recipes = conn
        .interact(move |conn| {
            recipe::table
                .filter(recipe::user_id.eq(user_id))
                .load::<Recipe>(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok(Json(recipes))
}

pub async fn get_recipe(
    State(pool): State<Pool>,
    Path(id): Path<i32>,
) -> Result<Json<Recipe>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let user = conn
        .interact(move |conn| recipe::table.filter(recipe::id.eq(id)).first::<Recipe>(conn))
        .await
        .map_err(internal_error)?
        .map_err(|_| (StatusCode::NOT_FOUND, "Recipe not found".to_string()))?;

    Ok(Json(user))
}
