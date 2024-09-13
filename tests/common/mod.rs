use cooking_book::db::{connect, DBConnection};

use diesel_migrations::{
    embed_migrations, EmbeddedMigrations, HarnessWithOutput, MigrationHarness,
};
use rstest::fixture;
use std::fs;
use std::path::Path;
use uuid::Uuid;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[fixture]
pub fn create_database_for_test() -> (DBConnection, String) {
    let test_db_dir = Path::new("test_db");

    if !test_db_dir.is_dir() {
        assert!(fs::create_dir(test_db_dir).is_ok());
    }

    let id = Uuid::new_v4();

    let database_path = test_db_dir.join(format!("cooking_book_test_{id}.db"));

    // Establish the connection to the database and make it mutable.
    let pool = connect(database_path.to_str().unwrap());
    let mut connection = DBConnection(pool.get().unwrap());

    let stdout = std::io::stdout();

    // Pass the mutable reference to the harness.
    let mut harness = HarnessWithOutput::new(&mut *connection, stdout);

    // Run pending migrations (requires mutable connection).
    harness.run_pending_migrations(MIGRATIONS).unwrap();

    // Return the connection and database path.
    (
        connection,
        database_path.into_os_string().into_string().unwrap(),
    )
}
