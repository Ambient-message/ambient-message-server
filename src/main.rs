use std::env;
use std::net::TcpListener;

use actix_web::dev::Server;


use infrastructure::server::*;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    server()?.await
}