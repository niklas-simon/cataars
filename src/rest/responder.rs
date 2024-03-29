use rocket::{Responder, serde::json::Json};

use super::Error;

#[derive(Responder)]
pub enum DefaultResponder<T> {
    #[response(status = 200)]
    Ok(Json<T>),
    #[response(status = 400)]
    BadRequest(Json<Error>),
    #[response(status = 404)]
    NotFound(Json<Error>),
    #[response(status = 500)]
    Error(Json<Error>)
}