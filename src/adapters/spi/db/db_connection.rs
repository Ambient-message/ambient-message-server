

use di::{injectable, Ref};
use diesel::{pg::PgConnection, r2d2::ConnectionManager};
use options::Options;
use serde::*;


type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;


#[derive(Default, Deserialize)]
pub struct DbOptions {
    #[serde(alias = "DatabaseUrl")]
    pub database_url: String,
}

#[injectable(DbContext)]
pub struct DbConnection {
    options: Ref<dyn Options<DbOptions>>,
}

pub trait DbContext {
    fn get_pool(&self) -> DbPool;
    fn database_url(&self) -> String;
}


impl DbContext for DbConnection {
    fn database_url(&self) -> String {
        self.options.value().database_url.clone()
    }

    fn get_pool(&self) -> DbPool {

        r2d2::Pool::new(
            ConnectionManager::<PgConnection>::new(
                &self.options.value().database_url.clone())
        ).expect("Failed to create database pool")
    }
}

