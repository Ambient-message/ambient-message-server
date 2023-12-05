use diesel::{pg::PgConnection, r2d2::ConnectionManager};
use dotenvy::dotenv;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct DbConnection {
    pub database_url: String,
}


impl DbConnection {
    pub fn get_pool(&self) -> DbPool {

        let manager = ConnectionManager::<PgConnection>::new(&self.database_url);

        r2d2::Pool::new(manager).unwrap()
    }
}