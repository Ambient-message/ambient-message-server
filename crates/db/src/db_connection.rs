use diesel::{pg::PgConnection, r2d2, r2d2::ConnectionManager};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct DbConnection {
    pub db_pool: DbPool,
}

impl DbConnection {
    pub fn new(db_name: String) -> Self {
        let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let database = format!("{}/{}", database_url, db_name);

        let manager = ConnectionManager::<PgConnection>::new(&database);

        DbConnection {
            db_pool: r2d2::Pool::new(manager).unwrap(),
        }
    }
}