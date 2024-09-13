use cooking_book::models::{CartWithRecipesOut, Data, RecipeWithIngredientsOut};
use cooking_book::response::{Errors, HTTPError};

use rocket::http::Status;
use rocket::local::blocking::Client;
use rstest::{fixture, rstest};
use serde_json::json;
use time::format_description::well_known::Rfc3339;
use time::{Duration, OffsetDateTime};

mod common;
use common::client;

#[rstest]
fn create_retrieve_delete_cart_test(client: Client) {
    let create_cart_response = client.post("/carts").dispatch();

    assert_eq!(create_cart_response.status(), Status::Created);

    let cart_with_recipe_out = create_cart_response
        .into_json::<Data<CartWithRecipesOut>>()
        .unwrap();

    assert!(
        OffsetDateTime::parse(&cart_with_recipe_out.data.created_at, &Rfc3339).unwrap()
            - OffsetDateTime::now_utc()
            < Duration::seconds(1)
    );
    assert_eq!(cart_with_recipe_out.data.recipes.len(), 0);

    let cart_id = cart_with_recipe_out.data.id;

    let retrieve_cart_response = client.get(format!("/carts/{cart_id}")).dispatch();

    assert_eq!(retrieve_cart_response.status(), Status::Ok);
    assert_eq!(
        retrieve_cart_response
            .into_json::<Data<CartWithRecipesOut>>()
            .unwrap(),
        cart_with_recipe_out
    );

    let delete_cart_response = client.delete(format!("/carts/{cart_id}")).dispatch();

    assert_eq!(delete_cart_response.status(), Status::NoContent);
}

#[rstest]
fn retrieve_non_existing_cart_test(client: Client) {
    let retrieve_cart_response = client.get("/carts/1").dispatch();

    assert_eq!(retrieve_cart_response.status(), Status::NotFound);
}

#[rstest]
fn delete_non_existing_cart_test(client: Client) {
    let delete_cart_response = client.delete("/carts/1").dispatch();

    assert_eq!(delete_cart_response.status(), Status::NotFound);
}

#[rstest]
fn add_recipe_to_cart_test(client: Client) {
    let create_cart_response = client.post("/carts").dispatch();

    assert_eq!(create_cart_response.status(), Status::Created);

    let mut cart = create_cart_response
        .into_json::<Data<CartWithRecipesOut>>()
        .unwrap();

    let create_recipe_response = client
        .post("/recipes")
        .json(&json!({"name": "Recette 1", "ingredients": [], "steps": []}))
        .dispatch();

    assert_eq!(create_recipe_response.status(), Status::Created);

    let recipe = create_recipe_response
        .into_json::<Data<RecipeWithIngredientsOut>>()
        .unwrap();

    let add_recipe_to_cart_response = client
        .post(format!(
            "/carts/{}/recipes/{}",
            cart.data.id, recipe.data.id
        ))
        .dispatch();

    assert_eq!(add_recipe_to_cart_response.status(), Status::Created);

    cart.data.recipes = vec![recipe.data];

    assert_eq!(
        add_recipe_to_cart_response
            .into_json::<Data<CartWithRecipesOut>>()
            .unwrap(),
        cart
    );
}

#[fixture]
fn create_cart_and_recipe(client: Client) -> (i32, i32, Client) {
    let create_cart_response = client.post("/carts").dispatch();

    assert_eq!(create_cart_response.status(), Status::Created);

    let cart_id = create_cart_response
        .into_json::<Data<CartWithRecipesOut>>()
        .unwrap()
        .data
        .id;

    let create_recipe_response = client
        .post("/recipes")
        .json(&json!({"name": "Recette", "ingredients": [], "steps": []}))
        .dispatch();

    let recipe_id = create_recipe_response
        .into_json::<Data<RecipeWithIngredientsOut>>()
        .unwrap()
        .data
        .id;

    (cart_id, recipe_id, client)
}

#[rstest]
fn add_non_existing_recipe_to_cart_test(create_cart_and_recipe: (i32, i32, Client)) {
    let (cart_id, recipe_id, client) = create_cart_and_recipe;

    let add_non_existing_recipe_to_cart = client
        .post(format!("/carts/{}/recipes/{}", cart_id, recipe_id + 1))
        .dispatch();

    assert_eq!(add_non_existing_recipe_to_cart.status(), Status::NotFound);
    assert_eq!(
        add_non_existing_recipe_to_cart
            .into_json::<Errors>()
            .unwrap(),
        Errors {
            errors: vec![HTTPError {
                status_code: Status::NotFound,
                message: format!("No recipe found with id {}", recipe_id + 1)
            }]
        }
    );
}

#[rstest]
fn add_recipe_to_non_existing_cart_test(create_cart_and_recipe: (i32, i32, Client)) {
    let (cart_id, recipe_id, client) = create_cart_and_recipe;

    let add_recipe_to_non_existing_cart = client
        .post(format!("/carts/{}/recipes/{}", cart_id + 1, recipe_id))
        .dispatch();

    assert_eq!(add_recipe_to_non_existing_cart.status(), Status::NotFound);
    assert_eq!(
        add_recipe_to_non_existing_cart
            .into_json::<Errors>()
            .unwrap(),
        Errors {
            errors: vec![HTTPError {
                status_code: Status::NotFound,
                message: format!("No cart found with id {}", cart_id + 1)
            }]
        }
    );
}

#[rstest]
fn add_existing_recipe_to_cart_test(create_cart_and_recipe: (i32, i32, Client)) {
    let (cart_id, recipe_id, client) = create_cart_and_recipe;

    let add_recipe_to_cart_response = client
        .post(format!("/carts/{}/recipes/{}", cart_id, recipe_id))
        .dispatch();

    assert_eq!(add_recipe_to_cart_response.status(), Status::Created);

    let add_existing_recipe_to_cart_response = client
        .post(format!("/carts/{}/recipes/{}", cart_id, recipe_id))
        .dispatch();

    assert_eq!(
        add_existing_recipe_to_cart_response.status(),
        Status::Conflict
    );
    assert_eq!(
        add_existing_recipe_to_cart_response
            .into_json::<Errors>()
            .unwrap(),
        Errors {
            errors: vec![HTTPError {
                status_code: Status::Conflict,
                message: format!("Recipe with id {recipe_id} is already in cart with id {cart_id}")
            }]
        }
    );
}
