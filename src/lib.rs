use std::net::TcpListener;

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod adapters;

pub fn run(listener: TcpListener, db_name: &str) -> Result<(), std::io::Error> {
    Ok(())
}
