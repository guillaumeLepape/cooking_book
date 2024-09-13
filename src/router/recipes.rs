use crate::db::DBConnection;
use crate::db_utils::{fetch_all_recipes, fetch_one_recipe, insert_recipe};
use crate::models::{Data, RecipeIn, RecipeWithIngredientsOut};
use crate::response::{conflict, created, internal_server_error, not_found_error, ok, HttpResult};

use diesel::result::{DatabaseErrorKind, Error as DieselError};
use rocket::serde::json::Json;

#[rocket::post("/", data = "<recipe_in>")]
pub fn create(
    recipe_in: Json<RecipeIn>,
    mut connection: DBConnection,
) -> HttpResult<RecipeWithIngredientsOut> {
    let recipe_inner = recipe_in.into_inner();

    match insert_recipe(&recipe_inner, &mut connection) {
        Ok(recipe) => Ok(created(Data { data: recipe })),
        Err(DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => Err(conflict(
            format!("Recipe already exists: {}", recipe_inner.name),
        )),
        Err(_) => Err(internal_server_error("Database error".to_owned())),
    }
}

#[rocket::get("/")]
pub fn retrieve_all(mut connection: DBConnection) -> HttpResult<Vec<RecipeWithIngredientsOut>> {
    fetch_all_recipes(&mut connection).map_or_else(
        |_| Err(internal_server_error("Database error".to_owned())),
        |recipes| Ok(ok(Data { data: recipes })),
    )
}

#[rocket::get("/<recipe_id>")]
pub fn retrieve(
    recipe_id: i32,
    mut connection: DBConnection,
) -> HttpResult<RecipeWithIngredientsOut> {
    match fetch_one_recipe(recipe_id, &mut connection) {
        Ok(recipe) => Ok(ok(Data { data: recipe })),
        Err(DieselError::NotFound) => Err(not_found_error(format!(
            "No recipe found with id {recipe_id}"
        ))),
        Err(_) => Err(internal_server_error("Database error".to_owned())),
    }
}
