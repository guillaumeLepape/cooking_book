use cooking_book::create_app;
use cooking_book::db;
use cooking_book::DATABASE_URL;

#[cfg(not(tarpaulin_include))]
#[must_use]
#[rocket::launch]
pub fn rocket() -> rocket::Rocket<rocket::Build> {
    create_app().manage(db::connect(DATABASE_URL))
}
