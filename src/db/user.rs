use crate::schema::user;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(serde::Serialize, Selectable, Queryable)]
#[diesel(table_name = user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub creation_time: NaiveDateTime,
    pub password: String,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = user)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}
