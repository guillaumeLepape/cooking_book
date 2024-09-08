use crate::db::DBConnection;
use crate::db_utils::{fetch_all_recipes, fetch_one_recipe, insert_recipe};
use crate::models::{Data, RecipeIn, RecipeWithIngredientsOut};
use crate::response::{created, internal_server_error, not_found_error, ok, HttpResult};

use diesel::result::Error as DieselError;
use rocket::serde::json::Json;

#[rocket::post("/", data = "<recipe_in>")]
pub fn create(
    recipe_in: Json<RecipeIn>,
    mut connection: DBConnection,
) -> HttpResult<RecipeWithIngredientsOut> {
    for ingredient in &recipe_in.ingredients {
        println!("{ingredient}");
    }

    insert_recipe(&recipe_in.into_inner(), &mut connection).map_or_else(
        |_| Err(internal_server_error("Database error".to_owned())),
        |res| Ok(created(Data { data: res })),
    )
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
            "No recipe found with id={recipe_id}"
        ))),
        Err(_) => Err(internal_server_error("Database error".to_owned())),
    }
}
