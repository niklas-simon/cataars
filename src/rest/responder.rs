use rocket::Responder;

#[derive(Responder)]
pub enum DefaultResponder<T> {
    #[response(status = 200)]
    Ok(T),
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 500)]
    Error(String)
}