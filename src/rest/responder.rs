use rocket::{Responder, serde::json::Json};

#[derive(Responder)]
pub enum DefaultResponder<T> {
    #[response(status = 200)]
    Ok(Json<T>),
    #[response(status = 500)]
    Error(String),
    #[response(status = 404)]
    NotFound(String)
}