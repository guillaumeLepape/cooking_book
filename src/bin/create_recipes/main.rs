use cooking_book::db;
use cooking_book::script;
use cooking_book::DATABASE_URL;

#[cfg(not(tarpaulin_include))]
fn main() {
    script::create_recipes(&mut db::DBConnection(
        db::connect(DATABASE_URL).get().unwrap(),
    ));
}
