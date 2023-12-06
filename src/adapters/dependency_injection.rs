use di::{ServiceCollection, ServiceLifetime, Injectable};
use super::{services::{user_service::UserService, service_builder::ServiceBuilder}, spi::db::db_connection::DbConnection};


impl ServiceBuilder {
    
    pub fn add_adapters(mut self) -> Self {
        self.services.add(UserService::inject(ServiceLifetime::Transient));
        self.services.add(DbConnection::inject(ServiceLifetime::Singleton));
        self
    }
  
}