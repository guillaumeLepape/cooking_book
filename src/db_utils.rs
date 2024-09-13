use crate::ingredient_parser::parse;
use crate::models::{
    Cart, CartRecipe, CartWithRecipesOut, Ingredient, IngredientOut, Recipe, RecipeIn,
    RecipeWithIngredientsOut, Step,
};
use crate::schema::{cart_recipes, carts, ingredients, recipes, steps};

use diesel::prelude::*;
use diesel::result::Error as DieselError;
use time::format_description::well_known::Rfc3339;

pub fn insert_recipe(
    recipe_in: &RecipeIn,
    connection: &mut SqliteConnection,
) -> Result<RecipeWithIngredientsOut, DieselError> {
    let inserted_recipes: Vec<Recipe> = match diesel::insert_into(recipes::table)
        .values(recipes::name.eq(&recipe_in.name))
        .get_results(connection)
    {
        Ok(res) => res,
        Err(error) => return Err(error),
    };

    let Some(recipe) = inserted_recipes.first() else {
        return Err(DieselError::NotFound);
    };

    let mut recipe_out = RecipeWithIngredientsOut {
        id: recipe.id,
        name: recipe.name.clone(),
        ingredients: Vec::with_capacity(recipe_in.ingredients.len()),
        steps: Vec::with_capacity(recipe_in.steps.len()),
    };

    for raw_ingredient in &recipe_in.ingredients {
        match insert_ingredient(recipe.id, raw_ingredient, connection) {
            Ok(ingredient) => recipe_out.ingredients.push(ingredient),
            Err(error) => return Err(error),
        };
    }

    for step in &recipe_in.steps {
        match insert_step(recipe.id, step.clone(), connection) {
            Ok(step) => recipe_out.steps.push(step),
            Err(error) => return Err(error),
        };
    }

    Ok(recipe_out)
}

fn insert_ingredient(
    recipe_id: i32,
    raw_ingredient: &str,
    connection: &mut SqliteConnection,
) -> Result<IngredientOut, DieselError> {
    let (name, preposition, quantity, unit) = parse(raw_ingredient).unwrap();

    let inserted_ingredients: Vec<Ingredient> = match diesel::insert_into(ingredients::table)
        .values((
            ingredients::recipe_id.eq(recipe_id),
            ingredients::preposition.eq(preposition),
            ingredients::name.eq(name),
            ingredients::quantity.eq(quantity),
            ingredients::unit.eq(unit),
        ))
        .get_results(connection)
    {
        Ok(res) => res,
        Err(error) => return Err(error),
    };

    let ingredient = inserted_ingredients.first().unwrap();

    Ok(IngredientOut {
        id: ingredient.id,
        preposition: ingredient.preposition.clone(),
        name: ingredient.name.clone(),
        quantity: ingredient.quantity,
        unit: ingredient.unit.clone(),
    })
}

fn insert_step(
    recipe_id: i32,
    step: String,
    connection: &mut SqliteConnection,
) -> Result<String, DieselError> {
    let inserted_steps: Vec<Step> = match diesel::insert_into(steps::table)
        .values((steps::recipe_id.eq(recipe_id), steps::description.eq(step)))
        .get_results(connection)
    {
        Ok(res) => res,
        Err(error) => return Err(error),
    };

    let step = inserted_steps.first().unwrap();

    Ok(step.description.clone())
}

pub fn fetch_one_recipe(
    recipe_id: i32,
    connection: &mut SqliteConnection,
) -> Result<RecipeWithIngredientsOut, DieselError> {
    let recipe = match recipes::table
        .filter(recipes::id.eq(recipe_id))
        .select(Recipe::as_select())
        .first(connection)
    {
        Ok(recipe) => recipe,
        Err(error) => return Err(error),
    };

    let ingredients_out = match fetch_recipe_ingredients(&recipe, connection) {
        Ok(ingredients) => ingredients,
        Err(error) => return Err(error),
    };

    let steps = match fetch_step_ingredients(&recipe, connection) {
        Ok(steps) => steps,
        Err(error) => return Err(error),
    };

    Ok(RecipeWithIngredientsOut {
        id: recipe.id,
        name: recipe.name,
        ingredients: ingredients_out,
        steps,
    })
}

fn fetch_recipe_ingredients(
    recipe: &Recipe,
    connection: &mut SqliteConnection,
) -> Result<Vec<IngredientOut>, DieselError> {
    let ingredients_out = match Ingredient::belonging_to(&recipe)
        .select(IngredientOut::as_select())
        .load(connection)
    {
        Ok(ingredients) => ingredients,
        Err(error) => return Err(error),
    };

    Ok(ingredients_out)
}

fn fetch_step_ingredients(
    recipe: &Recipe,
    connection: &mut SqliteConnection,
) -> Result<Vec<String>, DieselError> {
    let steps = match Step::belonging_to(&recipe)
        .select(Step::as_select())
        .load(connection)
    {
        Ok(steps) => steps,
        Err(error) => return Err(error),
    };

    Ok(steps.into_iter().map(|value| value.description).collect())
}

pub fn fetch_all_recipes(
    connection: &mut SqliteConnection,
) -> Result<Vec<RecipeWithIngredientsOut>, DieselError> {
    let recipes = match recipes::table.select(Recipe::as_select()).load(connection) {
        Ok(recipe) => recipe,
        Err(error) => return Err(error),
    };

    let mut recipes_with_ingredients = Vec::with_capacity(recipes.len());

    for recipe in recipes {
        let ingredients = match fetch_recipe_ingredients(&recipe, connection) {
            Ok(ingredients) => ingredients,
            Err(error) => return Err(error),
        };

        let steps = match fetch_step_ingredients(&recipe, connection) {
            Ok(steps) => steps,
            Err(error) => return Err(error),
        };

        recipes_with_ingredients.push(RecipeWithIngredientsOut {
            id: recipe.id,
            name: recipe.name,
            ingredients,
            steps,
        });
    }

    Ok(recipes_with_ingredients)
}

pub fn fetch_one_cart(
    cart_id: i32,
    connection: &mut SqliteConnection,
) -> Result<Cart, DieselError> {
    carts::table
        .filter(carts::id.eq(cart_id))
        .select(Cart::as_select())
        .first(connection)
}

pub fn fetch_one_cart_and_recipes(
    cart_id: i32,
    connection: &mut SqliteConnection,
) -> Result<CartWithRecipesOut, DieselError> {
    let cart = match fetch_one_cart(cart_id, connection) {
        Ok(cart) => cart,
        Err(error) => return Err(error),
    };

    let recipes = match CartRecipe::belonging_to(&cart)
        .inner_join(recipes::table)
        .select(Recipe::as_select())
        .load(connection)
    {
        Ok(recipes) => recipes,
        Err(error) => return Err(error),
    };

    let mut recipes_out = Vec::with_capacity(recipes.len());

    for recipe in recipes {
        match fetch_one_recipe(recipe.id, connection) {
            Ok(recipe) => recipes_out.push(recipe),
            Err(error) => return Err(error),
        };
    }

    let cart = CartWithRecipesOut {
        id: cart.id,
        created_at: cart.created_at.assume_utc().format(&Rfc3339).unwrap(),
        recipes: recipes_out,
    };

    Ok(cart)
}

pub fn insert_cart(connection: &mut SqliteConnection) -> Result<CartWithRecipesOut, DieselError> {
    let result: Vec<Cart> = match diesel::insert_into(carts::table)
        .default_values()
        .get_results(connection)
    {
        Ok(cart) => cart,
        Err(error) => return Err(error),
    };

    let Some(cart) = result.first() else {
        return Err(DieselError::NotFound);
    };

    Ok(CartWithRecipesOut {
        id: cart.id,
        created_at: cart.created_at.assume_utc().format(&Rfc3339).unwrap(),
        recipes: Vec::new(),
    })
}

pub fn delete_from_cart(
    cart_id: i32,
    connection: &mut SqliteConnection,
) -> Result<usize, DieselError> {
    diesel::delete(carts::table.filter(carts::id.eq(cart_id))).execute(connection)
}

pub fn insert_into_cart(
    cart_id: i32,
    recipe_id: i32,
    connection: &mut SqliteConnection,
) -> Result<usize, DieselError> {
    diesel::insert_into(cart_recipes::table)
        .values((
            cart_recipes::cart_id.eq(cart_id),
            cart_recipes::recipe_id.eq(recipe_id),
        ))
        .execute(connection)
}
