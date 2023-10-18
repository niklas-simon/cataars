use rocket::launch;

mod rest;
mod repositories;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(repositories::stage())
        .attach(rest::stage())
}