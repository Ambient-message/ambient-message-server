use di::{ServiceCollection, ServiceLifetime, Injectable};

use crate::adapters::services::service_builder::ServiceBuilder;

use super::users::handlers::create_user_handler::CreateUserHandler;


impl ServiceBuilder {
    
    pub fn add_application(mut self) -> Self {
        self.services.add(CreateUserHandler::inject(ServiceLifetime::Transient));
        self
    }
  
}