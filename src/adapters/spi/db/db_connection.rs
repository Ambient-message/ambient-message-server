use std::env;
use di::{injectable};
use diesel::{pg::PgConnection, r2d2::ConnectionManager};


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
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);

        r2d2::Pool::new(manager).unwrap()
    }
}