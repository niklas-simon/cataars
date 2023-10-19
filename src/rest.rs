use rocket::{serde::{Serialize, json::Json}, get, fairing::AdHoc, routes};

use crate::rest::{responder::DefaultResponder, response::Response};

use self::response::Links;

mod response;
mod responder;
mod users;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Info {
    name: String,
    version: String,
    authors: Vec<String>
}

#[get("/")]
fn index() -> DefaultResponder<Json<Response<Info>>> {
    DefaultResponder::Ok(Json(Response::new(Info {
            name: String::from(env!("CARGO_PKG_NAME")),
            version: String::from(env!("CARGO_PKG_VERSION")),
            authors: env!("CARGO_PKG_AUTHORS").split(",").map(|s| String::from(s)).collect()
        }, Links::Root))
    )
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("REST stage", | rocket | async {
        rocket.mount("/", routes![index])
            .attach(users::stage())
    })
}