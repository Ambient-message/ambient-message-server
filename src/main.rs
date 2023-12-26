use std::env;
use std::net::TcpListener;

use actix_web::dev::Server;
use chrono::Local;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let environment_file;
    if let Ok(e) = env::var("ENV") {
        environment_file = format!(".env.{}", e);
    } else {
        environment_file = String::from(".env");
    }

    dotenv::from_filename(environment_file).ok();

    let address = String::from("127.0.0.1:8888");

    let listener = TcpListener::bind(address.clone()).expect("Failed to bind random port");
    let database_name = dotenv::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    let port = listener.local_addr().unwrap().port();
    let app_name = "ambient-message-server";

    println!("[{}]", Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
    println!("Server running on http://{}", address);
    println!("App Name: {}", app_name);
    println!("Database: {}", database_name);
    println!("Port: {}", port);

    run(listener, app_name)?.await
}

pub fn run(listener: TcpListener, app_name: &str) -> Result<Server, std::io::Error> {
    infrastructure::server(listener, app_name)
}