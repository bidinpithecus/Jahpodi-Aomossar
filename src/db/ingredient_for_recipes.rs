use crate::schema::recipe_ingredient;
use diesel::prelude::*;

#[derive(serde::Serialize, Selectable, Queryable, Insertable)]
#[diesel(table_name = recipe_ingredient)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewRecipeIngredient {
    pub recipe_id: i32,
    pub ingredient_id: i32,
    pub quantity: String,
}

#[derive(serde::Deserialize, serde::Serialize, Insertable)]
#[diesel(table_name = recipe_ingredient)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewRecipeIngredientWithoutRecipeId {
    pub ingredient_id: i32,
    pub quantity: String,
}
