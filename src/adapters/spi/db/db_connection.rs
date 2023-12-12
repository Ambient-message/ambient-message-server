use std::{default, rc::Rc};
use config::ext::ConfigurationBinder;
use di::{injectable, Ref};
use diesel::{pg::PgConnection, r2d2::ConnectionManager};
use options::Options;
use serde::*;
use serde_json::*;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;


#[derive(Default, Deserialize)]
pub struct DbOptions {
    #[serde(alias = "DatabaseUrl")]
    pub database_url: String,
}

#[injectable]
pub struct DbConnection {
    options: Ref<dyn Options<DbOptions>>,
}

impl DbConnection {
    pub fn database_url(&self) -> String {
        self.options.value().database_url.clone()
    }

    pub fn get_pool(&self) -> DbPool {

        r2d2::Pool::new(
            ConnectionManager::<PgConnection>::new(
                &self.options.value().database_url.clone())
        ).expect("Failed to create database pool")
    }
}

