use ambient_message_server_lib::adapters::spi::db::db_connection::{DbConnection, DbContext, self};
use ambient_message_server_lib::application::users::handlers::create_user_handler::CreateUserHandler;
use ambient_message_server_lib::application::users::requests::create_user_request::CreateUserRequest;
use ambient_message_server_lib::domain::user::user_repository_abstract::UserRepositoryAbstract;
use ambient_message_server_lib::infrastructure::repositories::user_repository::UserRepository;
use chrono::{DateTime, Utc};
use futures_util::{SinkExt, StreamExt};
use std::env;
use std::net::{IpAddr, SocketAddr};
use std::rc::Rc;
use std::time::SystemTime;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use di::*;


#[tokio::main]
async fn main() -> std::io::Result<()> {


    let provider = ServiceCollection::new()
      .add(DbConnection::inject(ServiceLifetime::Singleton))
      .add(UserRepository::inject(ServiceLifetime::Transient))
      .add(CreateUserHandler::inject(ServiceLifetime::Transient))
      .build_provider()
      .unwrap();

    let mut handler = provider.get_required::<CreateUserHandler>();

    handler.execute(CreateUserRequest::new("vlad", "lox"));

    Ok(())
}
