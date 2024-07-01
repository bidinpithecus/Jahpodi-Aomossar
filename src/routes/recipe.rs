use crate::db::answer::{Answer, FullAnswer};
use crate::db::ingredient_for_recipes::NewRecipeIngredient;
use crate::db::question::{FullQuestion, Question};
use crate::db::recipe::{FullRecipe, NewRecipe, NewRecipeWithIngredients, Recipe};
use crate::db::user::User;
use crate::schema::{rating, answer, ingredient, question, recipe, recipe_ingredient, user};
use crate::utils::internal_error;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use deadpool_diesel::postgres::Pool;
use diesel::associations::HasTable;
use diesel::prelude::*;
use crate::db::ingredient::{Ingredient, IngredientWithQuantity};
use crate::db::rating::{FullRating, Rating};

pub async fn create_recipe(
    State(pool): State<Pool>,
    Json(new_recipe_with_ingredients): Json<NewRecipeWithIngredients>,
) -> Result<Json<Recipe>, (StatusCode, String)> {

    if new_recipe_with_ingredients.ingredients.is_empty() {
        return Err((StatusCode::UNAUTHORIZED, "List of ingredients must not be empty".to_string()));
    }

    let conn = pool.get().await.map_err(internal_error)?;

    let user_exists: bool = conn
        .interact(move |conn| {
            diesel::select(diesel::dsl::exists(
                user::table.filter(user::id.eq(new_recipe_with_ingredients.user_id)),
            ))
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    if !user_exists {
        return Err((StatusCode::NOT_FOUND, "User not found".to_string()));
    }

    let ingredient_ids: Vec<i32> = new_recipe_with_ingredients
        .ingredients
        .iter()
        .map(|ingredient| ingredient.ingredient_id)
        .collect();

    let ingredients_passed_count = ingredient_ids.len() as i64;

    let ingredients_actual_count: i64 = conn
        .interact(move |conn| {
            use diesel::dsl::count_star;
            use diesel::prelude::*;

            ingredient::table
                .filter(ingredient::id.eq_any(&ingredient_ids))
                .select(count_star())
                .first(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    if ingredients_actual_count.ne(&ingredients_passed_count) {
        return Err((StatusCode::NOT_FOUND, "One or more ingredients not found".to_string()));
    }

    let new_recipe_without_ingredients = NewRecipe {
        title: new_recipe_with_ingredients.title.clone(),
        description: new_recipe_with_ingredients.description.clone(),
        directions: new_recipe_with_ingredients.directions.clone(),
        user_id: new_recipe_with_ingredients.user_id,
    };

    let recipe = conn
        .interact(move |conn| {
            diesel::insert_into(recipe::table)
                .values(&new_recipe_without_ingredients)
                .returning(Recipe::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    let recipe_id = recipe.id;

    for ingredient in &new_recipe_with_ingredients.ingredients {
        let new_recipe_ingredient = NewRecipeIngredient {
            recipe_id,
            ingredient_id: ingredient.ingredient_id,
            quantity: ingredient.quantity.clone(),
        };

        conn
            .interact(move |conn| {
                diesel::insert_into(recipe_ingredient::table)
                    .values(&new_recipe_ingredient)
                    .execute(conn)
            })
            .await
            .map_err(internal_error)?
            .map_err(internal_error)?;
    }

    Ok(Json(recipe))
}

pub async fn list_recipes(
    State(pool): State<Pool>,
) -> Result<Json<Vec<Recipe>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| recipe::table.select(Recipe::as_select()).load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

pub async fn get_recipes_by_user_id(
    State(pool): State<Pool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<Recipe>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let user_exists: bool = conn
        .interact(move |conn| {
            diesel::select(diesel::dsl::exists(
                user::table.filter(user::id.eq(user_id)),
            ))
            .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    if !user_exists {
        return Err((StatusCode::NOT_FOUND, "User not found".to_string()));
    }

    let recipes = conn
        .interact(move |conn| {
            recipe::table
                .filter(recipe::user_id.eq(user_id))
                .load::<Recipe>(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok(Json(recipes))
}

pub async fn get_recipe(
    State(pool): State<Pool>,
    Path(id): Path<i32>,
) -> Result<Json<Recipe>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let user = conn
        .interact(move |conn| {
            recipe::table
                .filter(recipe::id.eq(id))
                .first::<Recipe>(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(|_| (StatusCode::NOT_FOUND, "Recipe not found".to_string()))?;

    Ok(Json(user))
}

pub async fn get_full_recipe(
    State(pool): State<Pool>,
    Path(id): Path<i32>,
) -> Result<Json<FullRecipe>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let recipe = conn
        .interact(move |conn| {
            recipe::table
                .filter(recipe::id.eq(id))
                .first::<Recipe>(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(|_| (StatusCode::NOT_FOUND, "Recipe not found".to_string()))?;

    let recipe_ingredients = conn
        .interact(move |conn| {
            recipe_ingredient::table
                .filter(recipe_ingredient::recipe_id.eq(id))
                .load::<NewRecipeIngredient>(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(|_| (StatusCode::NOT_FOUND, "Recipe not found".to_string()))?;

    let mut ingredients = Vec::new();
    for recipe_ingredient in recipe_ingredients {
        let ingredient = conn.interact(move |conn| {
           ingredient::table
               .filter(ingredient::id.eq(recipe_ingredient.ingredient_id))
               .first::<Ingredient>(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(|_| (StatusCode::NOT_FOUND, "Author not found".to_string()))?;

        ingredients.push(IngredientWithQuantity {
            name: ingredient.name,
            quantity: recipe_ingredient.quantity
        });
    }

    let recipe_author = conn
        .interact(move |conn| {
            user::table
                .filter(user::id.eq(recipe.user_id))
                .first::<User>(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(|_| (StatusCode::NOT_FOUND, "Author not found".to_string()))?;

    let questions = conn
        .interact(move |conn| {
            question::table
                .filter(question::recipe_id.eq(recipe.id))
                .load::<Question>(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    let mut full_questions = Vec::new();
    for question in questions {
        let question_author = conn
            .interact(move |conn| {
                user::table
                    .filter(user::id.eq(question.user_id))
                    .first::<User>(conn)
            })
            .await
            .map_err(internal_error)?
            .map_err(|_| {
                (
                    StatusCode::NOT_FOUND,
                    "Question author not found".to_string(),
                )
            })?;

        let answers = conn
            .interact(move |conn| {
                answer::table
                    .filter(answer::question_id.eq(question.id))
                    .load::<Answer>(conn)
            })
            .await
            .map_err(internal_error)?
            .map_err(internal_error)?;

        let mut full_answers = Vec::new();
        for answer in answers {
            let answer_author = conn
                .interact(move |conn| {
                    user::table
                        .filter(user::id.eq(answer.user_id))
                        .first::<User>(conn)
                })
                .await
                .map_err(internal_error)?
                .map_err(|_| (StatusCode::NOT_FOUND, "Answer author not found".to_string()))?;

            full_answers.push(FullAnswer {
                body: answer.body,
                date: answer.date,
                question_id: answer.question_id,
                user: answer_author,
            });
        }

        full_questions.push(FullQuestion {
            title: question.title,
            body: question.body,
            date: question.date,
            user: question_author,
            answers: full_answers,
        });
    }

    let ratings = conn
        .interact(move |conn| {
            rating::table
                .filter(rating::user_id.eq(recipe.id))
                .load::<Rating>(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    let mut full_ratings = Vec::new();
    for rating in ratings {
        let rating_author = conn
            .interact(move |conn| {
                user::table
                    .filter(user::id.eq(rating.user_id))
                    .first::<User>(conn)
            })
            .await
            .map_err(internal_error)?
            .map_err(|_| (StatusCode::NOT_FOUND, "Answer author not found".to_string()))?;

        full_ratings.push(FullRating {
            recipe_id: rating.recipe_id,
            score: rating.score,
            user: rating_author
        });
    }

    let full_recipe = FullRecipe {
        id: recipe.id,
        title: recipe.title,
        description: recipe.description,
        creation_date: recipe.creation_date,
        directions: recipe.directions,
        author: recipe_author,
        questions: full_questions,
        ingredients,
        ratings: full_ratings
    };

    Ok(Json(full_recipe))
}
