use chrono::{DateTime, Utc};
use futures_util::{SinkExt, StreamExt};
use std::env;
use std::net::{IpAddr, SocketAddr};
use std::rc::Rc;
use std::time::SystemTime;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use uuid::Uuid;
use ::inject::*;

struct Connection(isize);

impl Connection {
    #[inject]
    fn new(foo: isize) -> Self {
        Self(foo)
    }
}


struct Instance(isize);

impl Instance {
    #[inject]
    fn new(conn: &Connection) -> Self {
        Self(conn.0)
    }
}





#[tokio::main]
async fn main() -> std::io::Result<()> {
   
    Ok(())
}
