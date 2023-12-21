use std::env;
use std::net::TcpListener;

use actix_web::dev::Server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let environment_file;
    if let Ok(e) = env::var("ENV") {
        environment_file = format!(".env.{}", e);
    } else {
        environment_file = String::from(".env");
    }

    dotenv::from_filename(environment_file).ok();

    let listener = TcpListener::bind("0.0.0.0:8888").expect("Failed to bind random port");
    let database_name = dotenv::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

    run(listener, &database_name)?.await
}

pub fn run(listener: TcpListener, db_name: &str) -> Result<Server, std::io::Error> {
    infrastructure::server(listener, db_name)
}