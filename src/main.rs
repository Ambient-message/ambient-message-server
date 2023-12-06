
use ambient_message_server_lib::adapters::services::user_service::UserService;
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

// async fn handel_connection(stream: TcpStream) {
//     if let Ok(ws_stream) = accept_async(stream).await {
//         println!("WebSocket connection established");

//         let (mut write, mut read)
//             = ws_stream.split();

//         while let Some(Ok(mut msg)) = read.next().await {
//             if msg.is_text() {
//                 let data: Request = serde_json::from_str(msg.to_string().as_str()).expect("Invalid request");

//                 match data {
//                     Request::AddUser(_) => {}
//                     Request::GetUser(id) => {
//                         let user_handler =
//                             application::users::requests::by_id_request::ByIdRequest::new(
//                                 Box::new(infrastructure::repositories::user_repository::UserRepository {})
//                             );

//                         let user = user_handler.handler(id).unwrap();
//                         let v = serde_json::to_string(&user).unwrap();

//                         msg = tungstenite::Message::Text(v);
//                     }
//                 }

//                 if write.send(msg).await.is_err() {
//                     break;
//                 }
//             }
//         }
//     }

//     println!("WebSocket connection closed");
// }


#[tokio::main]
async fn main() -> std::io::Result<()> {

    let provider = ServiceCollection::new()
      .add(DbConnection::inject(ServiceLifetime::Singleton))
      .add(UserRepository::inject(ServiceLifetime::Transient))
      .add(CreateUserHandler::inject(ServiceLifetime::Transient))
      .add(UserService::inject(ServiceLifetime::Transient))
      .build_provider()
      .unwrap();

    let mut service = provider.get_required::<UserService>();

    service.save(CreateUserRequest::new("stas", "test"));

    Ok(())
}
