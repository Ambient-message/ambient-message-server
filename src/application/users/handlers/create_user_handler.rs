use std::rc::Rc;
use crate::domain::user::user::User;
use crate::domain::user::user_repository::UserRepository;

use crate::application::requests::{
    CreateUserRequest
};

pub struct CreateUserHandler {
    user_repository: Rc<dyn UserRepository>,
}


impl CreateUserHandler {
    pub fn new(user_repository: Rc<dyn UserRepository>) -> CreateUserHandler {
        CreateUserHandler { clieuser_repositorynt_repository }
    }

    pub fn execute(&self, request: CreateUserRequest) {
        let user = CreateUser::new(
            request.username.as_str(),
            request.password.as_str(),
        );
        
        self.user_repository.save(user);
    }
}