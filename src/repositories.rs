use rocket::{fairing::AdHoc, error};
use rocket_db_pools::Database;
use sqlx::migrate::Migrator;

pub mod users;

#[derive(Database)]
#[database("cataars")]
pub struct CatDB(sqlx::MySqlPool);

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("REST stage", | rocket | async {
        rocket.attach(CatDB::init())
            .attach(AdHoc::try_on_ignite("SQLx Migrations", | rocket | async {
                match CatDB::fetch(&rocket) {
                    Some(db) => match Migrator::new(std::path::Path::new("./migrations")).await {
                        Ok(migrator) => match migrator.run(&**db).await {
                            Ok(_) => Ok(rocket),
                            Err(e) => {
                                error!("Failed to initialize SQLx database: {}", e);
                                Err(rocket)
                            }
                        },
                        Err(e) => {
                            error!("Failed to initialize Migrator: {}", e);
                            Err(rocket)
                        }
                    }
                    None => Err(rocket),
                }
            }))
    })
}