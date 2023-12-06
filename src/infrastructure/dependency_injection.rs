use di::{ServiceCollection, ServiceLifetime, Injectable};

use crate::adapters::services::service_builder::ServiceBuilder;

use super::repositories::user_repository::UserRepository;



impl ServiceBuilder {
    
    pub fn add_infrastructure(mut self) -> Self {
        self.services.add(UserRepository::inject(ServiceLifetime::Transient));
        self
    }
  
}