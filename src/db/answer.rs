use crate::schema::answer;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::db::user::User;

#[derive(serde::Serialize, Selectable, Queryable)]
#[diesel(table_name = answer)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Answer {
    pub id: i32,
    pub body: String,
    pub date: NaiveDateTime,
    pub user_id: i32,
    pub question_id: i32,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = answer)]
pub struct NewAnswer {
    body: String,
    date: NaiveDateTime,
    pub user_id: i32,
    pub question_id: i32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct FullAnswer {
    pub body: String,
    pub date: NaiveDateTime,
    pub question_id: i32,
    pub user: User,
}
