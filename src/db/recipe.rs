use crate::schema::recipe;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(serde::Serialize, Selectable, Queryable)]
#[diesel(table_name = recipe)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Recipe {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub creation_date: NaiveDateTime,
    pub directions: String,
    pub user_id: i32
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = recipe)]
pub struct NewRecipe {
    pub title: String,
    pub description: String,
    pub directions: String,
}
