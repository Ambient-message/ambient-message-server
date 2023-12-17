use adapters::db::Db;
use diesel::{pg::PgConnection, prelude::*, r2d2, r2d2::ConnectionManager};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct DbConnection {}

impl DbConnection {
    pub fn get_pool() -> DbPool {
        let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let manager = ConnectionManager::<PgConnection>::new(&database_url);

        r2d2::Pool::new(manager).unwrap()
    }

    pub fn establish_connection() -> PgConnection {
        let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");

        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
}

impl Db for DbConnection {}