// @generated automatically by Diesel CLI.

diesel::table! {
    cart_recipes (cart_id, recipe_id) {
        cart_id -> Integer,
        recipe_id -> Integer,
    }
}

diesel::table! {
    carts (id) {
        id -> Integer,
        created_at -> Timestamp,
    }
}

diesel::table! {
    ingredients (id) {
        id -> Integer,
        recipe_id -> Integer,
        preposition -> Text,
        name -> Text,
        quantity -> Float,
        unit -> Text,
    }
}

diesel::table! {
    recipes (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    steps (id) {
        id -> Integer,
        recipe_id -> Integer,
        description -> Text,
    }
}

diesel::joinable!(cart_recipes -> carts (cart_id));
diesel::joinable!(cart_recipes -> recipes (recipe_id));
diesel::joinable!(ingredients -> recipes (recipe_id));
diesel::joinable!(steps -> recipes (recipe_id));

diesel::allow_tables_to_appear_in_same_query!(cart_recipes, carts, ingredients, recipes, steps,);
