use db::db::db_connection::DbConnection;

pub fn run() {
    let db = DbConnection{};
    cli::run(db);
}