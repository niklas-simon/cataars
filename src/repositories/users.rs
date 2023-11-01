use rocket_db_pools::{Connection, sqlx};
use rocket::{futures::{TryStreamExt, TryFutureExt}, serde::{Serialize, Deserialize}};
use sqlx::{Row, Error, mysql::{MySqlRow, MySqlQueryResult}};

use crate::repositories::CatDB;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub public: bool,
    pub admin: bool,
    pub link: String
}

impl User {
    pub fn new(id: i32, username: String, password: String, public: bool, admin: bool) -> User {
        let mut link = String::from("/users/");
        link.push_str(id.to_string().as_str());
        User {
            id,
            username,
            password,
            public,
            admin,
            link
        }
    }
    fn from(row: MySqlRow) -> User {
        User::new(
            row.get("id"),
            row.get("username"),
            row.get("password"),
            row.get("public"),
            row.get("admin")
        )
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

pub async fn create(db: &mut Connection<CatDB>, user: &User) -> Result<MySqlQueryResult, Error> {
    sqlx::query("INSERT INTO users (username, password, public) VALUES (?, ?, ?)")
        .bind(&user.username)
        .bind(&user.password)
        .bind(user.public)
        .execute(&mut **db)
        .await
}

pub async fn update(db: &mut Connection<CatDB>, user: &User) -> Result<MySqlQueryResult, Error> {
    sqlx::query("UPDATE users SET username = ?, password = ?, public = ?, admin = ? WHERE id = ?")
        .bind(&user.username)
        .bind(&user.password)
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

pub async fn get_by_name(db: &mut Connection<CatDB>, username: &String) -> Result<Option<User>, Error> {
    sqlx::query("SELECT * FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(&mut **db)
        .map_ok(|res| res.map(User::from))
        .await
}