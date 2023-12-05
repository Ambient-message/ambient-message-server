use ambient_message_server_lib::adapters::spi::db::db_connection::{self, DbConnection};
use ambient_message_server_lib::application::users::handlers::create_user_handler::{self, CreateUserHandler};
use ambient_message_server_lib::application::users::requests::create_user_request::CreateUserRequest;
use ambient_message_server_lib::domain::user::user::User;
use ambient_message_server_lib::domain::user::user_repository_abstract::{UserRepositoryAbstract, MockUserRepositoryAbstract};
use ambient_message_server_lib::infrastructure::repositories::user_repository::{UserRepository, self};
use chrono::{DateTime, Utc};
use dotenvy::dotenv;
use uuid::Uuid;
use std::time::{SystemTime};
use std::env;
use std::net::{SocketAddr, IpAddr};
use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use std::rc::Rc;



#[tokio::main]
async fn main() -> std::io::Result<()> {

    let environment_file;
    if let Ok(e) = env::var("ENV") {
        environment_file = format!(".env.{}", e);
    } else {
        environment_file = String::from(".env");
    }

    dotenvy::from_filename(environment_file).ok();

    //    println!("--------------------------------------------------------------------------------------");
    //
    //    let host : IpAddr = env::var("SERVER_HOST").unwrap_or_else(|_| String::from("127.0.0.1")).parse().expect("Invalid IP address");
    //    let port : u16 = env::var("SERVER_PORT").unwrap_or_else(|_| String::from("8080")).parse().expect("Invalid port number");
    //    let debug_mode = env::var("DEBUG_MODE").is_ok();
    //    let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");
    //    let database_name = dotenvy::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    //
    //    let addr = SocketAddr::from((host, port));
    //    let listener = TcpListener::bind(addr).await?;
    //
    //    println!("Starting server on http://{}:{}/", addr.ip(), addr.port());
    //    println!("Database URL: {}", database_url);
    //    println!("Database Name: {}", database_name);
    //    println!("Debug mode: {}", debug_mode);

    let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_connection = DbConnection{database_url : database_url};

    let user_repository = Rc::new(UserRepository::new(db_connection));

    let create_user_request = CreateUserRequest::new("john_doe", "password123");
    let create_user_handler = CreateUserHandler::new(user_repository.clone());

    create_user_handler.execute(create_user_request);

//    let start_time = SystemTime::now();
//    let formatted_start_time: DateTime<Utc> = start_time.into();
//    println!("Server started at {}", formatted_start_time);
//
//    println!("--------------------------------------------------------------------------------------");

    Ok(())
}

