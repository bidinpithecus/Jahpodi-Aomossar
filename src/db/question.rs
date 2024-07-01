use crate::schema::question;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::db::answer::FullAnswer;
use crate::db::user::User;

#[derive(serde::Serialize, Selectable, Queryable)]
#[diesel(table_name = question)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Question {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub date: NaiveDateTime,
    pub user_id: i32,
    pub recipe_id: i32,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = question)]
pub struct NewQuestion {
    title: String,
    body: String,
    date: NaiveDateTime,
    pub user_id: i32,
    pub recipe_id: i32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct FullQuestion {
    pub title: String,
    pub body: String,
    pub date: NaiveDateTime,
    pub user: User,
    pub answers: Vec<FullAnswer>,
}
