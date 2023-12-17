// use actix_web::{App, HttpServer, Responder, web};
// use actix_web::dev::Server;
// use adapters::{
//     self,
//     spi::db::db_connection::DbConnection,
// };
//
// async fn index() -> impl Responder {
//     "aaaaaaaaaaaaaaaaaaaaaaa"
// }
//
//
// pub fn server() -> Result<Server, std::io::Error> {
//
//     let server = HttpServer::new(|| App::new().route("/", web::get().to(index)))
//     .bind(("127.0.0.1", 8080)).unwrap()
//     .run();
//
//     println!("Server running on port {}", 8080);
//
//     Ok(server)
// }