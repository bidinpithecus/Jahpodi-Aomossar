use crate::schema::ingredient;
use diesel::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, Selectable, Queryable)]
#[diesel(table_name = ingredient)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = ingredient)]
pub struct NewIngredient {
    pub name: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct IngredientWithQuantity {
    pub name: String,
    pub quantity: String,
}