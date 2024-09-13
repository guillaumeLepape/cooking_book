use cooking_book::db::DBConnection;
use cooking_book::db_utils::fetch_all_recipes;
use cooking_book::script::create_recipes;

use rstest::rstest;

mod common;
use common::create_database_for_test;

#[rstest]
fn test_create_recipes(create_database_for_test: (DBConnection, String)) {
    let (mut connection, _) = create_database_for_test;

    create_recipes(&mut connection);

    let all_recipes = fetch_all_recipes(&mut connection).unwrap();

    assert_eq!(all_recipes.len(), 3);

    let recipe_names: Vec<String> = all_recipes.iter().map(|r| r.name.clone()).collect();

    assert_eq!(
        recipe_names,
        vec![
            "Saucisses aux lentilles",
            "Gratin de gnocchi au saumon et épinards",
            "Tapenade : la meilleure recette"
        ]
    );
}
