use rocket_db_pools::{Connection, sqlx};
use rocket::{futures::{TryStreamExt, TryFutureExt}, serde::Serialize};
use sqlx::{Row, Error, mysql::MySqlRow};

use crate::repositories::CatDB;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    id: i32,
    username: String,
    password: String,
    public: bool,
    admin: bool
}

impl User {
    fn from(row: MySqlRow) -> User {
        User {
            id: row.get("id"),
            username: row.get("username"),
            password: row.get("password"),
            public: row.get("public"),
            admin: row.get("admin")
        }
    }
}

pub async fn list(mut db: Connection<CatDB>) -> Result<Vec<User>, Error> {
    sqlx::query("SELECT * FROM users")
        .fetch(&mut **db)
        .map_ok(User::from)
        .try_collect::<Vec<User>>()
        .await
}

pub async fn detail(mut db: Connection<CatDB>, id: i32) -> Result<Option<User>, Error> {
    sqlx::query("SELECT * FROM users WHERE id = ?").bind(id)
        .fetch_optional(&mut **db)
        .map_ok(|res| res.map(User::from))
        .await
}