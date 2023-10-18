use rocket::serde::Serialize;
use std::collections::HashMap;

pub enum Links {
    Root,
    Users
}

impl Links {
    fn as_map(&self) -> HashMap<&'static str, &'static str> {
        match self {
            Self::Root => HashMap::from([
                ("cats", "/cats"),
                ("tags", "/tags"),
                ("users", "/users"),
                ("login", "/login"),
                ("images", "/images")
            ]),
            Self::Users => HashMap::from([
                ("user", "/user/:id")
            ])
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response<T> {
    data: T,
    links: HashMap<&'static str, &'static str>
}

impl<T> Response<T> {
    pub fn new(data: T, links: Links) -> Response<T> {
        Response {
            data,
            links: links.as_map(),
        }
    }
}