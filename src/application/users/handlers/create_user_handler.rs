use std::rc::Rc;
use crate::domain::user::user::User;
use crate::domain::user::user_repository::UserRepository;

use crate::application::users::requests::create_user_request::{
    CreateUserRequest
};

pub struct CreateUserHandler {
    user_repository: Rc<dyn UserRepository>,
}


impl CreateUserHandler {
    pub fn new(user_repository: Rc<dyn UserRepository>) -> CreateUserHandler {
        CreateUserHandler { user_repository }
    }

    pub fn execute(&self, request: CreateUserRequest) {

        let id = 1;

        let user = User::new(
            id,
            request.username.as_str(),
            request.password.as_str(),
        );
        
        self.user_repository.save(user);
    }
}