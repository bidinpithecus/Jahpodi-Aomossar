use crate::db::user::{AuthUser, NewUser, RegularUser, User};
use crate::schema::{regular_user, user};
use crate::utils::internal_error;
use axum::response::Html;
use axum::{extract::State, http::StatusCode, response::Json};
use bcrypt::{hash, verify};
use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;

pub async fn register(
    State(pool): State<Pool>,
    Json(mut new_user): Json<NewUser>,
) -> Result<Json<i32>, (StatusCode, String)> {
    let password_hash = hash(&new_user.password, bcrypt::DEFAULT_COST).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Password hashing failed".to_string(),
        )
    })?;

    new_user.password = password_hash;

    let conn = pool.get().await.map_err(internal_error)?;
    let user: User = conn
        .interact(move |conn| {
            diesel::insert_into(user::table)
                .values(&new_user)
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    let conn = pool.get().await.map_err(internal_error)?;
    let reg_user: RegularUser = conn
        .interact(move |conn| {
            diesel::insert_into(regular_user::table)
                .values(RegularUser {
                    id: user.id
                })
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;


    Ok(Json(reg_user.id))
}

pub async fn show_register_page() -> Html<String> {
    let html_content = tokio::fs::read_to_string("static/register.html").await.unwrap();
    Html(html_content)
}

pub async fn sign_in(
    State(pool): State<Pool>,
    Json(auth_user): Json<AuthUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let stored_user = conn
        .interact(move |conn| {
            user::table
                .filter(user::email.eq(&auth_user.email))
                .first::<User>(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                "Invalid username or password".to_string(),
            )
        })?;

    let password_match = verify(&auth_user.password, &stored_user.password).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Password verification failed".to_string(),
        )
    })?;

    if !password_match {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Invalid username or password".to_string(),
        ));
    }

    Ok(Json(stored_user))
}

pub async fn show_login_page() -> Html<String> {
    let html_content = tokio::fs::read_to_string("static/login.html").await.unwrap();
    Html(html_content)
}

pub async fn show_recipes_page() -> Html<String> {
    let html_content = tokio::fs::read_to_string("static/recipes.html").await.unwrap();
    Html(html_content)
}