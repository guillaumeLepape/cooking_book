use crate::db_utils::{
    delete_from_cart, fetch_one_cart, fetch_one_cart_and_recipes, insert_cart, insert_into_cart,
};
use crate::models::{CartWithRecipesOut, Data};
use crate::response::{
    created, internal_server_error, no_content, not_found_error, ok, EmptyHttpResult, HttpResult,
};

use crate::db::DBConnection;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error as DieselError;

#[rocket::post("/")]
pub fn create(mut connection: DBConnection) -> HttpResult<CartWithRecipesOut> {
    insert_cart(&mut connection).map_or_else(
        |_| Err(internal_server_error("Database error".to_owned())),
        |cart| Ok(created(Data { data: cart })),
    )
}

#[rocket::get("/<cart_id>")]
pub fn retrieve(cart_id: i32, mut connection: DBConnection) -> HttpResult<CartWithRecipesOut> {
    match fetch_one_cart_and_recipes(cart_id, &mut connection) {
        Ok(cart) => Ok(ok(Data { data: cart })),
        Err(DieselError::NotFound) => {
            Err(not_found_error(format!("No cart found with id {cart_id}")))
        }
        Err(_) => Err(internal_server_error("Database error".to_owned())),
    }
}

#[rocket::delete("/<cart_id>")]
pub fn delete(cart_id: i32, mut connection: DBConnection) -> EmptyHttpResult {
    let Ok(deleted_records) = delete_from_cart(cart_id, &mut connection) else {
        return Err(internal_server_error("Database error".to_owned()));
    };

    if deleted_records == 0 {
        return Err(not_found_error(format!("No cart found with id {cart_id}")));
    }

    Ok(no_content())
}

#[rocket::post("/<cart_id>/recipes/<recipe_id>")]
pub fn add_recipe(
    cart_id: i32,
    recipe_id: i32,
    mut connection: DBConnection,
) -> HttpResult<CartWithRecipesOut> {
    match insert_into_cart(cart_id, recipe_id, &mut connection) {
        Ok(res) => res,
        Err(DieselError::DatabaseError(DatabaseErrorKind::ForeignKeyViolation, _)) => {
            if fetch_one_cart(cart_id, &mut connection).is_err() {
                return Err(not_found_error(format!("No cart found with id {cart_id}")));
            }

            return Err(not_found_error(format!(
                "No recipe found with id {recipe_id}"
            )));
        }
        Err(_) => {
            return Err(internal_server_error("Database error".to_owned()));
        }
    };

    let Ok(cart_with_recipes) = fetch_one_cart_and_recipes(cart_id, &mut connection) else {
        return Err(internal_server_error("Database error".to_owned()));
    };

    Ok(ok(Data {
        data: cart_with_recipes,
    }))
}
