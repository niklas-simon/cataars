use rocket::{fairing::AdHoc, routes, get, serde::json::{Json, Value}, post, patch, delete};
use rocket_db_pools::Connection;
use crate::{repositories::{users::{self, User}, CatDB}, rest::response::Response, rest::responder::DefaultResponder};
use sha256::digest;

use super::response::Links;

#[get("/")]
async fn list(mut db: Connection<CatDB>) -> DefaultResponder<Json<Response<Vec<User>>>> {
    match users::list(&mut db).await {
        Ok(data) => DefaultResponder::Ok(Json(Response::new(data, Links::Users))),
        Err(err) => DefaultResponder::Error(err.to_string())
    }
}

#[get("/<id>")]
async fn detail(mut db: Connection<CatDB>, id: i32) -> DefaultResponder<Json<Response<User>>> {
    match users::detail(&mut db, id).await {
        Ok(Some(data)) => DefaultResponder::Ok(Json(Response::new(data, Links::Users))),
        Ok(None) => DefaultResponder::NotFound(String::from("User not found")),
        Err(err) => DefaultResponder::Error(err.to_string())
    }
}

#[post("/", data = "<user_json>")]
async fn create(mut db: Connection<CatDB>, user_json: Json<Value>) -> DefaultResponder<String> {
    let user = user_json.into_inner();
    let db_user = User {
        id: 0,
        username: if let Some(username) = user.get("username").and_then(|u| u.as_str()) { 
            String::from(username) 
        } else {
            return DefaultResponder::BadRequest(String::from("field username is required"))
        },
        password: if let Some(password) = user.get("password").and_then(|u| u.as_str()) { 
            digest(password) 
        } else {
            return DefaultResponder::BadRequest(String::from("field password is required"))
        },
        public: if let Some(public) = user.get("public").and_then(|u| u.as_bool()) { 
            public
        } else {
            return DefaultResponder::BadRequest(String::from("field public is required"))
        },
        admin: false
    };
    match users::create(&mut db, db_user).await {
        Ok(_) => DefaultResponder::Ok(String::from("Ok")),
        Err(err) => DefaultResponder::Error(err.to_string())
    }
}

#[patch("/<id>", data = "<user_json>")]
async fn update(mut db: Connection<CatDB>, id: i32, user_json: Json<Value>) -> DefaultResponder<String> {
    let user = user_json.into_inner();
    match users::detail(&mut db, id).await {
        Ok(Some(mut db_user)) => {
            if let Some(username) = user.get("username").and_then(|u| u.as_str()) {
                db_user.username = String::from(username);
            }
            if let Some(password) = user.get("password").and_then(|u| u.as_str()) {
                db_user.password = digest(password);
            }
            if let Some(public) = user.get("public").and_then(|u| u.as_bool()) {
                db_user.public = public;
            }
            if let Some(admin) = user.get("admin").and_then(|u| u.as_bool()) {
                db_user.admin = admin;
            }
            match users::update(&mut db, db_user).await {
                Ok(_) => DefaultResponder::Ok(String::from("Ok")),
                Err(err) => DefaultResponder::Error(err.to_string())
            }
        },
        Ok(None) => DefaultResponder::NotFound(String::from("user not found")),
        Err(err) => DefaultResponder::Error(err.to_string())
    }
}

#[delete("/<id>")]
async fn remove(mut db: Connection<CatDB>, id: i32) -> DefaultResponder<String> {
    match users::remove(&mut db, id).await {
        Ok(_) => DefaultResponder::Ok(String::from("Ok")),
        Err(err) => DefaultResponder::Error(err.to_string())
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("REST stage: users", | rocket | async {
        rocket.mount("/users", routes![list, detail, create, update, remove])
    })
}