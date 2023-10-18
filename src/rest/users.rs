use rocket::{fairing::AdHoc, routes, get, serde::json::Json};
use rocket_db_pools::Connection;
use crate::{repositories::{users::{self, User}, CatDB}, rest::response::Response, rest::responder::DefaultResponder};

use super::response::Links;

#[get("/")]
async fn list(db: Connection<CatDB>) -> DefaultResponder<Response<Vec<User>>> {
    match users::list(db).await {
        Ok(data) => DefaultResponder::Ok(Json(Response::new(data, Links::Users))),
        Err(err) => DefaultResponder::Error(err.to_string())
    }
}

#[get("/<id>")]
async fn detail(db: Connection<CatDB>, id: i32) -> DefaultResponder<Response<User>> {
    match users::detail(db, id).await {
        Ok(Some(data)) => DefaultResponder::Ok(Json(Response::new(data, Links::Users))),
        Ok(None) => DefaultResponder::NotFound(String::from("User not found")),
        Err(err) => DefaultResponder::Error(err.to_string())
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("REST stage: users", | rocket | async {
        rocket.mount("/users", routes![list, detail])
    })
}