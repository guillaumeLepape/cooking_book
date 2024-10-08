pub mod db;
pub mod db_utils;
pub mod ingredient_parser;
pub mod models;
pub mod response;
pub mod router;
pub mod schema;
pub mod script;

use crate::router::carts as cart_router;
use crate::router::recipes as recipe_router;

pub const DATABASE_URL: &str = "cooking_book.db";

#[must_use]
pub fn create_app() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount(
            "/api/carts",
            rocket::routes![
                cart_router::create,
                cart_router::retrieve,
                cart_router::delete,
                cart_router::add_recipe,
            ],
        )
        .mount(
            "/api/recipes",
            rocket::routes![
                recipe_router::create,
                recipe_router::retrieve_all,
                recipe_router::retrieve,
            ],
        )
}
