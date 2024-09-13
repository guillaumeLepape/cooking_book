use cooking_book::models::{Data, IngredientOut, RecipeWithIngredientsOut};
use cooking_book::response::{Errors, HTTPError};

use rocket::http::Status;
use rocket::local::blocking::Client;
use rstest::rstest;
use serde_json::json;

mod common;
use common::client;

#[rstest]
fn create_and_retrieve_recipe(client: Client) {
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
fn recipe_not_found_test(client: Client) {
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
fn recipe_already_exist_test(client: Client) {
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
    );

    assert_eq!(
        create_recipe_already_exist_response
            .into_json::<Errors>()
            .unwrap(),
        Errors {
            errors: vec![HTTPError {
                status_code: Status::Conflict,
                message: "Recipe already exists: Saumon fumé à la poele".to_string()
            }]
        }
    );
}

#[rstest]
fn recipe_fetch_all_recipes_test(client: Client) {
    let recipe_in1 = json!(
        {
            "name": "Recette 1",
            "ingredients": ["20g de sucre"],
            "steps": ["Etape 1"]
        }
    );

    let recipe_in2 = json!(
        {
            "name": "Recette 2",
            "ingredients": ["30mL de lait"],
            "steps": ["Etape 2"]
        }
    );

    assert_eq!(
        client
            .post("/recipes")
            .json(&recipe_in1)
            .dispatch()
            .status(),
        Status::Created
    );
    assert_eq!(
        client
            .post("/recipes")
            .json(&recipe_in2)
            .dispatch()
            .status(),
        Status::Created
    );

    let response_fetch_all_recipes = client.get("/recipes").dispatch();

    assert_eq!(response_fetch_all_recipes.status(), Status::Ok);
    assert_eq!(
        response_fetch_all_recipes
            .into_json::<Data<Vec<RecipeWithIngredientsOut>>>()
            .unwrap(),
        Data {
            data: vec![
                RecipeWithIngredientsOut {
                    id: 1,
                    name: "Recette 1".to_string(),
                    ingredients: vec![IngredientOut {
                        id: 1,
                        preposition: "de ".to_string(),
                        name: "sucre".to_string(),
                        quantity: 20.0,
                        unit: "g".to_string()
                    }],
                    steps: vec!["Etape 1".to_string()]
                },
                RecipeWithIngredientsOut {
                    id: 2,
                    name: "Recette 2".to_string(),
                    ingredients: vec![IngredientOut {
                        id: 2,
                        preposition: "de ".to_string(),
                        name: "lait".to_string(),
                        quantity: 30.0,
                        unit: "mL".to_string()
                    }],
                    steps: vec!["Etape 2".to_string()]
                }
            ]
        }
    );
}
