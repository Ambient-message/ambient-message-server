use diesel::{pg::PgConnection, r2d2, r2d2::ConnectionManager};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct DbConnection {
    pub db_pool: DbPool,
}

impl DbConnection {
    pub fn new(database_url: String) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(&database_url);

        //TODO Возвращать пул вместо одного соединения
        DbConnection {
            db_pool: r2d2::Pool::new(manager).unwrap(),
        }
    }
}