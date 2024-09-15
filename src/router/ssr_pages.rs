use rocket_dyn_templates::{context, Template};

use crate::db;

use crate::db_utils::{fetch_all_carts, fetch_all_recipes};
use crate::models::CartWithRecipesOut;

use time::format_description::well_known::Rfc3339;

#[rocket::get("/")]
pub fn index() -> Template {
    Template::render("index", context! { field: "value" })
}

#[rocket::get("/recipes")]
pub fn recipes(mut connection: db::DBConnection) -> Template {
    let mut recipes = fetch_all_recipes(&mut connection).unwrap();

    recipes.sort_by(|recipe1, recipe2| recipe1.name.cmp(&recipe2.name));

    Template::render("recipes", context! { recipes: recipes})
}

#[rocket::get("/carts")]
pub fn carts(mut connection: db::DBConnection) -> Template {
    let mut carts = fetch_all_carts(&mut connection).unwrap();

    carts.sort_by(|cart1, cart2| cart2.created_at.cmp(&cart1.created_at));

    let carts: Vec<CartWithRecipesOut> = carts
        .into_iter()
        .map(|cart| CartWithRecipesOut {
            id: cart.id,
            created_at: cart.created_at.assume_utc().format(&Rfc3339).unwrap(),
            recipes: vec![],
        })
        .collect();

    Template::render("carts", context! {carts: carts})
}
