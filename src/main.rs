mod domain;
mod application;
mod infrastructure;

use chrono::{DateTime, Utc};
use std::time::{SystemTime};
use std::env;
use std::net::{SocketAddr, IpAddr};
use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use domain::request::request::Request;


async fn handel_connection(stream: TcpStream) {
    if let Ok(ws_stream) = accept_async(stream).await {
        println!("WebSocket connection established");

        let (mut write, mut read)
            = ws_stream.split();

        while let Some(Ok(mut msg)) = read.next().await {
            if msg.is_text() {
                let data: Request = serde_json::from_str(msg.to_string().as_str()).expect("Invalid request");

                match data {
                    Request::AddUser(_) => {}
                    Request::GetUser(id) => {
                        let user_handler =
                            application::users::requests::by_id_request::ByIdRequest::new(
                                Box::new(infrastructure::repositories::user_repository::UserRepository {})
                            );

                        let user = user_handler.handler(id).unwrap();
                        let v = serde_json::to_string(&user).unwrap();

                        msg = tungstenite::Message::Text(v);
                    }
                }

                if write.send(msg).await.is_err() {
                    break;
                }
            }
        }
    }

    println!("WebSocket connection closed");
}


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let environment_file;
    if let Ok(e) = env::var("ENV") {
        environment_file = format!(".env.{}", e);
    } else {
        environment_file = String::from(".env");
    }

    dotenvy::from_filename(environment_file).ok();

    let host: IpAddr = env::var("SERVER_HOST").unwrap_or_else(|_| String::from("127.0.0.1")).parse().expect("Invalid IP address");
    let port: u16 = env::var("SERVER_PORT").unwrap_or_else(|_| String::from("8080")).parse().expect("Invalid port number");
    let debug_mode = env::var("DEBUG_MODE").is_ok();

    let addr = SocketAddr::from((host, port));
    let listener = TcpListener::bind(addr).await?;

    println!("Starting server on https://{}:{}/", addr.ip(), addr.port());
    println!("Debug mode: {}", debug_mode);

    let start_time = SystemTime::now();
    let formatted_start_time: DateTime<Utc> = start_time.into();
    println!("Server started at {}", formatted_start_time);

    println!("Press Enter to stop the server");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    while let Ok((stream, socket_addr)) = listener.accept().await {
        println!("Connect accepted {}", socket_addr);
        tokio::spawn(handel_connection(stream));
    }

    Ok(())
}

