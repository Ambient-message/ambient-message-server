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

        let manager = ConnectionManager::<PgConnection>::new("postgres://postgres:hiWGNPEmtNmtR4U@localhost/postgres");

        r2d2::Pool::new(manager).unwrap()
    }
}