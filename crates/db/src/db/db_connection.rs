use adapters::db::Db;
use diesel::{pg::PgConnection, prelude::*, r2d2, r2d2::ConnectionManager};

pub struct DbConnection {
    pub db_pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}

impl DbConnection {
    pub fn new() -> Self {
        let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let manager = ConnectionManager::<PgConnection>::new(&database_url);

        DbConnection {
            db_pool: r2d2::Pool::new(manager).unwrap()
        }
    }
}

impl Db for DbConnection {}