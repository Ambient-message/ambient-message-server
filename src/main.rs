use infrastructure::cli;

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {

//     server()?.await
// }

pub fn main() {
    cli::run();
}