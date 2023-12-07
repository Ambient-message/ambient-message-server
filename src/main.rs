use std::env;
use std::net::SocketAddr;
use dotenvy::dotenv;
use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use ambient_message_server_lib::adapters::services::service_builder::ServiceBuilder;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let builder = ServiceBuilder::new();

    let provider = builder
        .add_adapters()
        .add_application()
        .add_infrastructure()
        .build().build_provider().unwrap();

    dotenv().expect(".env file not found");

    let addr = format!("{}:{}",
                       env::var("SERVER_HOST").unwrap(),
                       env::var("SERVER_PORT").unwrap());

    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    println!("Listening on: {}", addr);

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr));
    }

    Ok(())
}

async fn handle_connection(stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    let (mut write, mut read) = ws_stream.split();

    while let Some(Ok(msg)) = read.next().await {
        match msg {
            Message::Text(text) => {
                println!("Received text: {}", text);
                // let mut service = provider.get_required::<UserService>();
                // let user = service.save(CreateUserRequest::new("stas", "loks"))
                //     .expect("Can`t create user");
                // let value = serde_json::to_string(&user).expect("Error during serialization");

                if let Err(e) = write.send(Message::Text(text)).await {
                    eprintln!("Error sending message: {:?}", e);
                    break;
                }
            }
            _ => {}
        }
    }

    println!("WebSocket connection closed");
}