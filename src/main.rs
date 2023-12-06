use ambient_message_server_lib::adapters::spi::db::db_connection::DbConnection;
use ambient_message_server_lib::infrastructure::repositories::user_repository::UserRepository;
use ambient_message_server_lib::application::users::handlers::create_user_handler::CreateUserHandler;
use ambient_message_server_lib::application::users::requests::create_user_request::CreateUserRequest;
use ambient_message_server_lib::adapters::services::user_service::UserService;
use di::*;


#[tokio::main]
async fn main() -> std::io::Result<()> {

    let provider = ServiceCollection::new()
      .add(DbConnection::inject(ServiceLifetime::Singleton))
      .add(UserRepository::inject(ServiceLifetime::Transient))
      .add(CreateUserHandler::inject(ServiceLifetime::Transient))
      .add(UserService::inject(ServiceLifetime::Transient))
      .build_provider()
      .unwrap();

    let service = provider.get_required::<UserService>();

    service.save(CreateUserRequest::new("stas", "test"));

    Ok(())
}
