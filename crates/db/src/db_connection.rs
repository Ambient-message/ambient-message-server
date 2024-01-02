use diesel::{pg::PgConnection, r2d2, r2d2::ConnectionManager};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct DbConnection {
    pub db_pool: DbPool,
}

impl DbConnection {
    pub fn new() -> Self {
        let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let manager = ConnectionManager::<PgConnection>::new(&database_url);

        //TODO Возвращать пул вместо одного соединения
        DbConnection {
            db_pool: r2d2::Pool::new(manager).unwrap(),
        }
    }
}