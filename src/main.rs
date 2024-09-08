use cooking_book::create_app;

#[must_use]
#[rocket::launch]
pub fn rocket() -> rocket::Rocket<rocket::Build> {
    create_app()
}
