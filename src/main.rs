use rocket::serde::{Serialize, json::Json};
#[macro_use] extern crate rocket;
mod response;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Info {
    name: String,
    version: String,
    authors: Vec<String>
}

#[get("/")]
fn index() -> Json<response::Response<Info>> {
    Json(response::Response::new(Info {
            name: String::from(env!("CARGO_PKG_NAME")),
            version: String::from(env!("CARGO_PKG_VERSION")),
            authors: env!("CARGO_PKG_AUTHORS").split(",").map(|s| String::from(s)).collect()
        }, response::Links::Root)
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}