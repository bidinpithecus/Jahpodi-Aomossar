use crate::db::ingredient_request::{IngredientRequest, NewIngredientRequest};
use crate::schema::ingredient_request;
use crate::utils::internal_error;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
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

pub async fn get_ingredient_request(
    State(pool): State<Pool>,
    Path(id): Path<i32>,
) -> Result<Json<IngredientRequest>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let user = conn
        .interact(move |conn| {
            ingredient_request::table
                .filter(ingredient_request::id.eq(id))
                .first::<IngredientRequest>(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(|_| (StatusCode::NOT_FOUND, "User not found".to_string()))?;

    Ok(Json(user))
}
