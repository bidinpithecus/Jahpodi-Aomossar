use crate::schema::rating;
use diesel::prelude::*;
use crate::db::user::User;

#[derive(serde::Serialize, serde::Deserialize, Selectable, Queryable, Insertable)]
#[diesel(table_name = rating)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Rating {
    pub score: i32,
    pub recipe_id: i32,
    pub user_id: i32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct FullRating {
    pub score: i32,
    pub recipe_id: i32,
    pub user: User,
}
