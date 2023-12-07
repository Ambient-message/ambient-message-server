use std::rc::Rc;

use crate::{
    application::users::{requests::create_user_request::CreateUserRequest,
                         handlers::create_user_handler::CreateUserHandler}};
use di::injectable;
use std::fmt::Error;
use std::sync::{Arc, Mutex};
use crate::domain::user::User;


#[injectable]
pub struct UserService {
    create_user_handler: Rc<CreateUserHandler>,
}


impl UserService {
    pub fn save(&self, create_user_request: CreateUserRequest) -> Result<User, Error> {
        self.create_user_handler.execute(create_user_request)
    }
}