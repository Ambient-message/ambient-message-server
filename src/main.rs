use std::io;
use std::net::SocketAddr;
use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;


#[tokio::main]
async fn main() -> io::Result<()> {
    let addr: SocketAddr = "127.0.0.1:9000".parse().unwrap();
    let listener = TcpListener::bind(addr).await?;
    println!("Server running on: {}", addr);

    while let Ok((stream, socket_addr)) = listener.accept().await {
        println!("Connect accepted {}", socket_addr);
        tokio::spawn(handel_connection(stream));
    }


    Ok(())
}

async fn handel_connection(stream: TcpStream) {
    if let Ok(ws_stream) = accept_async(stream).await {
        println!("WebSocket connection established");

        let (mut write, mut read)
            = ws_stream.split();

        while let Some(Ok(msg)) = read.next().await {
            if msg.is_text() || msg.is_binary() {
                println!("Try to send {}", msg);
                if write.send(msg).await.is_err() {
                    break;
                } else {}
            }
        }
    }

    println!("WebSocket connection closed");
}