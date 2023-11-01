use std::vec;

use rocket::{fairing::AdHoc, routes, get, serde::json::{Json, Value}, post, patch, delete};
use rocket_db_pools::Connection;
use crate::{repositories::{users::{self, User}, CatDB}, rest::responder::DefaultResponder, extract_from_json, set_if_exists_in_json};
use sha256::digest;

use super::Error;

#[get("/")]
async fn list(mut db: Connection<CatDB>) -> DefaultResponder<Vec<User>> {
    match users::list(&mut db).await {
        Ok(data) => DefaultResponder::Ok(Json(data)),
        Err(err) => DefaultResponder::Error(Json(Error::new_nofields(err.to_string())))
    }
}

#[get("/<id>")]
async fn detail(mut db: Connection<CatDB>, id: i32) -> DefaultResponder<User> {
    match users::detail(&mut db, id).await {
        Ok(Some(data)) => DefaultResponder::Ok(Json(data)),
        Ok(None) => DefaultResponder::NotFound(Json(Error::new(String::from("user not found"), vec![String::from("id")]))),
        Err(err) => DefaultResponder::Error(Json(Error::new_nofields(err.to_string())))
    }
}

#[post("/", data = "<user_json>")]
async fn create(mut db: Connection<CatDB>, user_json: Json<Value>) -> DefaultResponder<User> {
    let user = user_json.into_inner();
    let username = extract_from_json!(user, username, as_str, String::from, BadRequest, "field username is required");
    match users::get_by_name(&mut db, &username).await {
        Ok(Some(_)) => return DefaultResponder::BadRequest(Json(Error::new(String::from("already exists"), vec![String::from("username")]))),
        Err(err) => return DefaultResponder::Error(Json(Error::new_nofields(err.to_string()))),
        Ok(None) => ()
    };
    let db_user = User::new(
        0,
        username,
        extract_from_json!(user, password, as_str, digest, BadRequest, "field password is required"),
        extract_from_json!(user, public, as_bool, |v| v, BadRequest, "field public is required"),
        false
    );
    if let Err(err) = users::create(&mut db, &db_user).await {
        return DefaultResponder::Error(Json(Error::new_nofields(err.to_string())))
    };
    match users::get_by_name(&mut db, &db_user.username).await {
        Ok(Some(user)) => DefaultResponder::Ok(Json(user)),
        Ok(None) => DefaultResponder::Error(Json(Error::new_nofields(String::from("user not found but it has been created")))),
        Err(err) => DefaultResponder::Error(Json(Error::new_nofields(err.to_string())))
    }
}

#[patch("/<id>", data = "<user_json>")]
async fn update(mut db: Connection<CatDB>, id: i32, user_json: Json<Value>) -> DefaultResponder<User> {
    let user = user_json.into_inner();
    let mut db_user = match users::detail(&mut db, id).await {
        Ok(Some(db_user)) => db_user,
        Ok(None) => return DefaultResponder::NotFound(Json(Error::new(String::from("user not found"), vec![String::from("id")]))),
        Err(err) => return DefaultResponder::Error(Json(Error::new_nofields(err.to_string())))
    };
    set_if_exists_in_json!(user, &mut db_user, username, as_str, String::from);
    set_if_exists_in_json!(user, &mut db_user, password, as_str, digest);
    set_if_exists_in_json!(user, &mut db_user, public, as_bool, bool::from);
    set_if_exists_in_json!(user, &mut db_user, admin, as_bool, bool::from);
    if let Err(err) = users::update(&mut db, &db_user).await {
        return DefaultResponder::Error(Json(Error::new_nofields(err.to_string())))
    };
    DefaultResponder::Ok(Json(db_user))
}

#[delete("/<id>")]
async fn remove(mut db: Connection<CatDB>, id: i32) -> DefaultResponder<User> {
    let db_user = match users::detail(&mut db, id).await {
        Ok(Some(db_user)) => db_user,
        Ok(None) => return DefaultResponder::NotFound(Json(Error::new(String::from("user not found"), vec![String::from("id")]))),
        Err(err) => return DefaultResponder::Error(Json(Error::new_nofields(err.to_string())))
    };
    if let Err(err) = users::remove(&mut db, id).await {
        return DefaultResponder::Error(Json(Error::new_nofields(err.to_string())))
    };
    DefaultResponder::Ok(Json(db_user))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("REST stage: users", | rocket | async {
        rocket.mount("/users", routes![list, detail, create, update, remove])
    })
}