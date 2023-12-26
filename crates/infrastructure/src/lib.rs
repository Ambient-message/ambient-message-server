use std::env;
use std::net::TcpListener;

use actix_cors::Cors;
use actix_web::{dev::Server, web, App, HttpServer};

use adapters::api::shared::app_state::AppState;
use adapters::spi::user::user_repository::UserRepository;
use db::db_connection::DbConnection;

pub fn server(listener: TcpListener, db_name: &str, app_name: &str) -> Result<Server, std::io::Error> {

    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug");

    let db_connection = DbConnection::new(db_name.to_string());

    let data = web::Data::new(AppState {
        app_name: String::from(app_name),
        user_repository: UserRepository { db_connection },
    });

    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec!["Content-Type"]),
            )
            .app_data(data.clone())
            .configure(adapters::api::shared::routes::routes)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
