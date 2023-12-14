use std::{env, net::TcpListener};

use adapters::{
    self,
    spi::{
        db::{db_connection::DbConnection},
    },
};

use actix_web::{dev::Server, middleware::Logger};
use actix_web::{web, App, HttpServer, HttpResponse, Responder};


async fn index() -> impl Responder {
    "aaaaaaaaaaaaaaaaaaaaaaa"
}


pub fn server() -> Result<Server, std::io::Error> {

    let server = HttpServer::new(|| App::new().route("/", web::get().to(index)))
    .bind(("127.0.0.1", 8080)).unwrap()
    .run();

    println!("Server running on port {}", 8080);

    Ok(server)
}