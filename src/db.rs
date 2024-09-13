use rocket::http::Status;

use diesel::r2d2;
use diesel::sqlite::SqliteConnection;

use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest};
use rocket::{Request, State};

use std::ops::{Deref, DerefMut};

type ConnectionManager = r2d2::ConnectionManager<SqliteConnection>;

type Pool = r2d2::Pool<ConnectionManager>;

#[must_use]
pub fn connect(database_url: &str) -> Pool {
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(database_url);
    let Ok(pool) = r2d2::Pool::new(manager) else {
        panic!("Error connecting to the database");
    };

    pool
}

pub struct DBConnection(pub r2d2::PooledConnection<ConnectionManager>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DBConnection {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, ()> {
        let pool = request.guard::<&State<Pool>>().await;

        match pool {
            Outcome::Success(pool) => Outcome::Success(Self(pool.get().unwrap())),
            Outcome::Error(_) => Outcome::Error((Status::InternalServerError, ())),
            Outcome::Forward(_) => Outcome::Forward(Status::ServiceUnavailable),
        }
    }
}

impl Deref for DBConnection {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DBConnection {
    fn deref_mut(&mut self) -> &mut SqliteConnection {
        &mut self.0
    }
}
