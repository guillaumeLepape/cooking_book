use cooking_book::create_app;
use cooking_book::db;
use cooking_book::models::{Data, RecipeWithIngredientsOut};
use cooking_book::response::{Errors, HTTPError};

use rocket::http::Status;
use rocket::local::blocking::Client;
use rstest::rstest;
use serde_json::json;

mod common;
use common::create_database_for_test;

#[rstest]
fn create_and_retrieve_recipe(create_database_for_test: (db::DBConnection, String)) {
    let (_, database_path) = create_database_for_test;

    let client = Client::tracked(create_app().manage(db::connect(&database_path)))
        .expect("valid rocket instance");

    let create_recipe_response = client
        .post("/recipes")
        .json(&json!(
            {
                "name": "Saumon fumé à la poele",
                "ingredients": ["125 g de saumon fumé"],
                "steps": ["Mettre le saumon dans la poele. Cuire à feu doux pendant 10 minutes."]
            }
        ))
        .dispatch();

    assert_eq!(create_recipe_response.status(), Status::Created);

    let recipe_out = create_recipe_response
        .into_json::<Data<RecipeWithIngredientsOut>>()
        .unwrap();

    assert_eq!(recipe_out.data.name, "Saumon fumé à la poele".to_string());
    assert_eq!(recipe_out.data.ingredients.len(), 1);

    assert_eq!(
        recipe_out.data.ingredients[0].preposition,
        "de ".to_string()
    );
    assert_eq!(
        recipe_out.data.ingredients[0].name,
        "saumon fumé".to_string()
    );
    assert!((recipe_out.data.ingredients[0].quantity - 125.0).abs() < 0.0001);
    assert_eq!(recipe_out.data.ingredients[0].unit, "g".to_string());
    assert_eq!(
        recipe_out.data.steps,
        vec!["Mettre le saumon dans la poele. Cuire à feu doux pendant 10 minutes.".to_string()],
    );

    let recipe_id = recipe_out.data.id;

    let retrieve_recipe_response = client.get(format!("/recipes/{recipe_id}")).dispatch();
    assert_eq!(retrieve_recipe_response.status(), Status::Ok);
    assert_eq!(
        retrieve_recipe_response
            .into_json::<Data<RecipeWithIngredientsOut>>()
            .unwrap(),
        recipe_out
    );
}

#[rstest]
fn recipe_not_found_test(create_database_for_test: (db::DBConnection, String)) {
    let (_, database_path) = create_database_for_test;

    let client = Client::tracked(create_app().manage(db::connect(&database_path)))
        .expect("valid rocket instance");

    let not_found_recipe_response = client.get("/recipes/1").dispatch();

    assert_eq!(not_found_recipe_response.status(), Status::NotFound);
    assert_eq!(
        not_found_recipe_response.into_json::<Errors>().unwrap(),
        Errors {
            errors: vec![HTTPError {
                status_code: Status::NotFound,
                message: "No recipe found with id 1".to_string()
            }]
        }
    );
}

#[rstest]
fn recipe_already_exist_test(create_database_for_test: (db::DBConnection, String)) {
    let (_, database_path) = create_database_for_test;

    let client = Client::tracked(create_app().manage(db::connect(&database_path)))
        .expect("valid rocket instance");

    let recipe_in = json!(
        {
            "name": "Saumon fumé à la poele",
            "ingredients": ["125 g de saumon fumé"],
            "steps": ["Mettre le saumon dans la poele. Cuire à feu doux pendant 10 minutes."]
        }
    );

    let create_recipe_response = client.post("/recipes").json(&recipe_in).dispatch();

    assert_eq!(create_recipe_response.status(), Status::Created);

    let create_recipe_already_exist_response = client.post("/recipes").json(&recipe_in).dispatch();

    assert_eq!(
        create_recipe_already_exist_response.status(),
        Status::Conflict
    )
}
