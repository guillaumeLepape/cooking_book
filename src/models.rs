use crate::schema::{cart_recipes, carts, ingredients, recipes, steps};
use diesel::prelude::{Associations, Identifiable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RecipeIn {
    pub name: String,
    pub ingredients: Vec<String>,
    pub steps: Vec<String>,
}

#[derive(Queryable, Identifiable, Selectable)]
#[diesel(table_name = recipes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Recipe {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(Recipe))]
#[diesel(table_name = ingredients)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Ingredient {
    pub id: i32,
    pub recipe_id: i32,
    pub preposition: String,
    pub name: String,
    pub quantity: f32,
    pub unit: String,
}

#[derive(Debug, PartialEq, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = ingredients)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct IngredientOut {
    pub id: i32,
    pub preposition: String,
    pub name: String,
    pub quantity: f32,
    pub unit: String,
}

#[derive(Queryable, Identifiable, Selectable, Associations, Serialize)]
#[diesel(belongs_to(Recipe))]
#[diesel(table_name = steps)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Step {
    pub id: i32,
    pub recipe_id: i32,
    pub description: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct RecipeWithIngredientsOut {
    pub id: i32,
    pub name: String,
    pub ingredients: Vec<IngredientOut>,
    pub steps: Vec<String>,
}

#[derive(Queryable, Identifiable, Selectable)]
#[diesel(table_name = carts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Cart {
    pub id: i32,
    pub created_at: PrimitiveDateTime,
}

#[derive(Queryable, Identifiable, Selectable, Associations)]
#[diesel(primary_key(cart_id, recipe_id))]
#[diesel(belongs_to(Cart))]
#[diesel(belongs_to(Recipe))]
#[diesel(table_name = cart_recipes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CartRecipe {
    pub cart_id: i32,
    pub recipe_id: i32,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct CartWithRecipesOut {
    pub id: i32,
    pub created_at: String,
    pub recipes: Vec<RecipeWithIngredientsOut>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Data<T> {
    pub data: T,
}
