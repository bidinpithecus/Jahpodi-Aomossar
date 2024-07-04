use crate::schema::recipe;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::db::ingredient::IngredientWithQuantity;
use crate::db::ingredient_for_recipes::NewRecipeIngredientWithoutRecipeId;
use crate::db::question::FullQuestion;
use crate::db::rating::FullRating;
use crate::db::user::User;

#[derive(serde::Serialize, Selectable, Queryable)]
#[diesel(table_name = recipe)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Recipe {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub creation_date: NaiveDateTime,
    pub directions: String,
    pub user_id: i32,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = recipe)]
pub struct NewRecipe {
    pub title: String,
    pub description: String,
    pub directions: String,
    pub user_id: i32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct NewRecipeWithIngredients {
    pub title: String,
    pub description: String,
    pub directions: String,
    pub user_id: i32,
    pub ingredients: Vec<NewRecipeIngredientWithoutRecipeId>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct FullRecipe {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub creation_date: NaiveDateTime,
    pub directions: String,
    pub author: User,
    pub questions: Vec<FullQuestion>,
    pub ingredients: Vec<IngredientWithQuantity>,
    pub ratings: Vec<FullRating>
}
