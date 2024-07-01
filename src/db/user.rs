use crate::schema::{user, regular_user, admin};
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, Selectable, Queryable, Clone)]
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
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AuthUser {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize, Selectable, Queryable, Clone)]
#[diesel(table_name = user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserWithoutPassword {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub creation_time: NaiveDateTime,
}

#[derive(serde::Serialize, serde::Deserialize, Selectable, Queryable, Insertable)]
#[diesel(table_name = regular_user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RegularUser {
    pub id: i32,
}

#[derive(serde::Serialize, serde::Deserialize, Selectable, Queryable, Insertable)]
#[diesel(table_name = admin)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Admin {
    pub id: i32,
}
