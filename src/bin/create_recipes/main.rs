use cooking_book::db;
use cooking_book::script;
use cooking_book::DATABASE_URL;

fn main() {
    script::create_recipes(&mut db::DBConnection(
        db::connect(DATABASE_URL).get().unwrap(),
    ));
}
