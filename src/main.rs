use ambient_message_server_lib::adapters::services::service_builder::ServiceBuilder;
use ambient_message_server_lib::adapters::services::user_service::UserService;
use ambient_message_server_lib::application::users::requests::create_user_request::CreateUserRequest;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let builder = ServiceBuilder::new();

    let provider = builder
        .add_adapters()
        .add_application()
        .add_infrastructure()
        .build().build_provider().unwrap();


    let instance = provider.get_required::<UserService>();

    instance.save(CreateUserRequest::new("root", "1234"));
    Ok(())
}
