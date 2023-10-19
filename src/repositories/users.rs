use rocket_db_pools::{Connection, sqlx};
use rocket::{futures::{TryStreamExt, TryFutureExt}, serde::{Serialize, Deserialize}};
use sqlx::{Row, Error, mysql::{MySqlRow, MySqlQueryResult}};

use crate::repositories::CatDB;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    #[serde(default)]
    pub id: i32,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub public: bool,
    #[serde(default)]
    pub admin: bool
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

pub async fn list(db: &mut Connection<CatDB>) -> Result<Vec<User>, Error> {
    sqlx::query("SELECT * FROM users")
        .fetch(&mut **db)
        .map_ok(User::from)
        .try_collect::<Vec<User>>()
        .await
}

pub async fn detail(db: &mut Connection<CatDB>, id: i32) -> Result<Option<User>, Error> {
    sqlx::query("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(&mut **db)
        .map_ok(|res| res.map(User::from))
        .await
}

pub async fn create(db: &mut Connection<CatDB>, user: User) -> Result<MySqlQueryResult, Error> {
    sqlx::query("INSERT INTO users (username, password, public) VALUES (?, ?, ?)")
        .bind(user.username)
        .bind(user.password)
        .bind(user.public)
        .execute(&mut **db)
        .await
}

pub async fn update(db: &mut Connection<CatDB>, user: User) -> Result<MySqlQueryResult, Error> {
    sqlx::query("UPDATE users SET username = ?, password = ?, public = ?, admin = ? WHERE id = ?")
        .bind(user.username)
        .bind(user.password)
        .bind(user.public)
        .bind(user.admin)
        .bind(user.id)
        .execute(&mut **db)
        .await
}

pub async fn remove(db: &mut Connection<CatDB>, id: i32) -> Result<MySqlQueryResult, Error> {
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(&mut **db)
        .await
}