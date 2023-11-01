use std::collections::HashMap;

use rocket::{serde::{Serialize, json::Json}, get, fairing::AdHoc, routes};

use crate::rest::responder::DefaultResponder;

mod responder;
mod users;

#[macro_export] macro_rules! extract_from_json {
    ($obj:expr, $value:ident, $type:ident, $transform:expr, $err_type:ident, $err:literal) => {
        if let Some($value) = $obj.get(stringify!($value)).and_then(|u| u.$type()) { 
            $transform($value) 
        } else {
            return DefaultResponder::$err_type(Json(Error::new(String::from($err), vec![String::from(stringify!($value))])))
        }
    };
}

#[macro_export] macro_rules! set_if_exists_in_json {
    ($obj:expr, $db_obj:expr, $value:ident, $type:ident, $transform:expr) => {
        if let Some($value) = $obj.get(stringify!($value)).and_then(|u| u.$type()) {
            $db_obj.$value = $transform($value);
        }
    };
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Info {
    name: String,
    version: String,
    authors: Vec<String>,
    links: HashMap<&'static str, &'static str>
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Error {
    message: String,
    fields: Vec<String>
}

impl Error {
    pub fn new_nofields(message: String) -> Error {
        Error {
            message,
            fields: Vec::new()
        }
    }

    pub fn new(message: String, fields: Vec<String>) -> Error {
        Error {
            message,
            fields
        }
    }
}

#[get("/")]
fn index() -> DefaultResponder<Info> {
    DefaultResponder::Ok(Json(Info {
            name: String::from(env!("CARGO_PKG_NAME")),
            version: String::from(env!("CARGO_PKG_VERSION")),
            authors: env!("CARGO_PKG_AUTHORS").split(",").map(|s| String::from(s)).collect(),
            links: HashMap::from([
                ("cats", "/cats"),
                ("images", "/images"),
                ("login", "/login"),
                ("tags", "/tags"),
                ("users", "/users")
            ])
        })
    )
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("REST stage", | rocket | async {
        rocket.mount("/", routes![index])
            .attach(users::stage())
    })
}