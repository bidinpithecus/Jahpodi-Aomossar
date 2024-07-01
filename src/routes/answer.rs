use crate::db::answer::{NewAnswer, Answer};
use crate::schema::{answer, recipe, user};
use crate::utils::internal_error;
use axum::{extract::State, http::StatusCode, response::Json};
use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;

pub async fn answer_question(
    State(pool): State<Pool>,
    Json(new_answer): Json<NewAnswer>,
) -> Result<Json<Answer>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let user_exists: bool = conn
        .interact(move |conn| {
            diesel::select(diesel::dsl::exists(
                user::table.filter(user::id.eq(new_answer.user_id)),
            ))
            .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    if !user_exists {
        return Err((StatusCode::NOT_FOUND, "User not found".to_string()));
    }

    let question_exists: bool = conn
        .interact(move |conn| {
            diesel::select(diesel::dsl::exists(
                recipe::table.filter(recipe::id.eq(new_answer.question_id)),
            ))
            .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    if !question_exists {
        return Err((StatusCode::NOT_FOUND, "Question not found".to_string()));
    }

    let res = conn
        .interact(|conn| {
            diesel::insert_into(answer::table)
                .values(new_answer)
                .returning(Answer::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

pub async fn get_answers_by_question_id(
    State(pool): State<Pool>,
    Json(question_id): Json<i32>,
) -> Result<Json<Vec<Answer>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let answers = conn
        .interact(move |conn| {
            answer::table
                .filter(answer::question_id.eq(question_id))
                .load::<Answer>(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok(Json(answers))
}
