use rocket::http::Status;

use diesel::r2d2;
use diesel::sqlite::SqliteConnection;

use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest};
use rocket::{Request, State};

use dotenvy::dotenv;
use std::env;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct DBConnectionError;

pub type ConnectionManager = r2d2::ConnectionManager<SqliteConnection>;

pub type Pool = r2d2::Pool<ConnectionManager>;

pub fn connect() -> Result<Pool, DBConnectionError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL");

    let Ok(database_url) = database_url else {
        return Err(DBConnectionError);
    };

    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(database_url);
    let Ok(pool) = r2d2::Pool::new(manager) else {
        return Err(DBConnectionError);
    };

    Ok(pool)
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
