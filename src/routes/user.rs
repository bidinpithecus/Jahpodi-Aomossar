use crate::db::user::{NewUser, User};
use crate::schema::user;
use crate::utils::internal_error;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;

pub async fn create_user(
    State(pool): State<Pool>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| {
            diesel::insert_into(user::table)
                .values(new_user)
                .returning(User::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

pub async fn list_users(State(pool): State<Pool>) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| user::table.select(User::as_select()).load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

pub async fn get_user(
    State(pool): State<Pool>,
    Path(id): Path<i32>,
) -> Result<Json<User>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let user = conn
        .interact(move |conn| user::table.filter(user::id.eq(id)).first::<User>(conn))
        .await
        .map_err(internal_error)?
        .map_err(|_| (StatusCode::NOT_FOUND, "User not found".to_string()))?;

    Ok(Json(user))
}
