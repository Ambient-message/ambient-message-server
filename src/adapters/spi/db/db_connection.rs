use std::env;

use di::{injectable, inject};
use diesel::{pg::PgConnection, r2d2::ConnectionManager};
use dotenvy::dotenv;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[injectable(DbContext)]
#[derive(Default)]
pub struct DbConnection {
    pub database_url: String,
}


pub trait DbContext {
    fn get_pool(&self) -> DbPool;
}

impl DbContext for DbConnection {

    fn get_pool(&self) -> DbPool {

        let environment_file;
        if let Ok(e) = env::var("ENV") {
            environment_file = format!(".env.{}", e);
        } else {
            environment_file = String::from(".env");
        }

        dotenvy::from_filename(environment_file).ok();

        let manager = ConnectionManager::<PgConnection>::new(env::var("DATABASE_URL").expect("DATABASE_URL must be set"));

        r2d2::Pool::new(manager).unwrap()
    }
}