use std::{default, rc::Rc};
use config::ext::ConfigurationBinder;
use di::{injectable, Ref};
use diesel::{pg::PgConnection, r2d2::ConnectionManager};
use options::Options;
use serde::*;
use serde_json::*;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Deserialize, Default)]
pub struct DbConnectionOptions {
    pub database_url: String,
}

#[injectable(DbContext)]
pub struct DbConnection {
    options: Ref<dyn Options<DbConnectionOptions>>,
}
pub trait DbContext {
    fn get_pool(&self) -> DbPool;
}

impl DbContext for DbConnection {
    fn get_pool(&self) -> DbPool {

        r2d2::Pool::new(
            ConnectionManager::<PgConnection>::new(
                &self.options.value().database_url)
        ).expect("Failed to create database pool")
    }
}