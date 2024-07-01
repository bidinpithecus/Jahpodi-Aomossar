use crate::db::question::{NewQuestion, Question};
use crate::schema::{question, recipe, user};
use crate::utils::internal_error;
use axum::{extract::State, http::StatusCode, response::Json};
use axum::extract::Path;
use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;

pub async fn ask_question(
    State(pool): State<Pool>,
    Json(new_question): Json<NewQuestion>,
) -> Result<Json<Question>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let user_exists: bool = conn
        .interact(move |conn| {
            diesel::select(diesel::dsl::exists(
                user::table.filter(user::id.eq(new_question.user_id)),
            ))
            .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    if !user_exists {
        return Err((StatusCode::NOT_FOUND, "User not found".to_string()));
    }

    let recipe_exists: bool = conn
        .interact(move |conn| {
            diesel::select(diesel::dsl::exists(
                recipe::table.filter(recipe::id.eq(new_question.recipe_id)),
            ))
            .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    if !recipe_exists {
        return Err((StatusCode::NOT_FOUND, "Recipe not found".to_string()));
    }

    let res = conn
        .interact(|conn| {
            diesel::insert_into(question::table)
                .values(new_question)
                .returning(Question::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

pub async fn get_questions_by_recipe_id(
    State(pool): State<Pool>,
    Path(recipe_id): Path<i32>,
) -> Result<Json<Vec<Question>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let questions = conn
        .interact(move |conn| {
            question::table
                .filter(question::recipe_id.eq(recipe_id))
                .load::<Question>(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok(Json(questions))
}
