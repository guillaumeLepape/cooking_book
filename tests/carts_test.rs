use cooking_book::models::{CartWithRecipesOut, Data};

use rocket::http::Status;
use rocket::local::blocking::Client;
use rstest::rstest;
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
