use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::models::Data;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HTTPError {
    pub status_code: Status,
    pub message: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Errors {
    pub errors: Vec<HTTPError>,
}

#[derive(Responder)]
pub enum ErrorResponder {
    // #[response(status = 400)]
    // Unauthorized(Json<T>),
    #[response(status = 404)]
    NotFound(Json<Errors>),
    #[response(status = 409)]
    Conflict(Json<Errors>),
    #[response(status = 500)]
    InternalServerError(Json<Errors>),
}

#[must_use]
pub fn not_found_error(error_message: String) -> ErrorResponder {
    ErrorResponder::NotFound(Json(Errors {
        errors: vec![HTTPError {
            status_code: Status::NotFound,
            message: error_message,
        }],
    }))
}

#[must_use]
pub fn conflict(error_message: String) -> ErrorResponder {
    ErrorResponder::Conflict(Json(Errors {
        errors: vec![HTTPError {
            status_code: Status::Conflict,
            message: error_message,
        }],
    }))
}

#[must_use]
pub fn internal_server_error() -> ErrorResponder {
    ErrorResponder::InternalServerError(Json(Errors {
        errors: vec![HTTPError {
            status_code: Status::InternalServerError,
            message: "Database error".to_owned(),
        }],
    }))
}

#[derive(Responder)]
pub enum SuccessResponder<T> {
    #[response(status = 200)]
    Ok(Json<T>),
    #[response(status = 201)]
    Created(Json<T>),
    #[response(status = 204)]
    NoContent(()),
}

#[must_use]
pub const fn ok<T>(data: T) -> SuccessResponder<T> {
    SuccessResponder::Ok(Json(data))
}

#[must_use]
pub const fn created<T>(data: T) -> SuccessResponder<T> {
    SuccessResponder::Created(Json(data))
}

#[must_use]
pub const fn no_content() -> SuccessResponder<()> {
    SuccessResponder::NoContent(())
}

pub type HttpResult<T> = Result<SuccessResponder<Data<T>>, ErrorResponder>;
pub type EmptyHttpResult = Result<SuccessResponder<()>, ErrorResponder>;
