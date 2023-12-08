use ambient_message_server_lib::adapters::services::service_builder::ServiceBuilder;
use ambient_message_server_lib::adapters::services::user_service::UserService;
use ambient_message_server_lib::application::users::requests::create_user_request::CreateUserRequest;
use ambient_message_server_lib::adapters::spi::db::db_connection::{DbConnection, DbContext, self, DbConnectionOptions};
use ambient_message_server_lib::application::users::handlers::create_user_handler::CreateUserHandler;
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
use config::{*, ext::*};
use di::*;
use options::{*, ext::*};

#[tokio::main]
async fn main() -> std::io::Result<()> {


    let config : Rc<dyn Configuration> =Rc::from(
        DefaultConfigurationBuilder::new()
            .add_json_file("appsettings.json")
            .build()
            .unwrap()
            .as_config());

    let builder = ServiceBuilder::new();

    let provider = builder
    .add_adapters()
    .add_infrastructure()
    .add_application()
    .build()
    .apply_config_at::<DbConnectionOptions>(config, "data")
    .build_provider().unwrap();


    let service = provider.get_required::<UserService>();

   service.save(CreateUserRequest::new("stas", "test"));

    Ok(())
}
