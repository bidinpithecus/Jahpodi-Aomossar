use crate::schema::ingredient_request;
use diesel::prelude::*;

#[derive(serde::Serialize, Selectable, Queryable)]
#[diesel(table_name = ingredient_request)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct IngredientRequest {
    pub id: i32,
    pub name: String,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = ingredient_request)]
pub struct NewIngredientRequest {
    pub name: String,
}
