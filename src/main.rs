#![allow(unused_imports)]

use std::env;
use std::net::TcpListener;

use actix_web::dev::Server;
use infrastructure::cli;


use infrastructure::server::*;


// #[actix_web::main]
// async fn main() -> std::io::Result<()> {

//     server()?.await
// }

pub fn main() {
    cli::run();
}