use std::rc::Rc;

use crate::{
    application::users::{requests::create_user_request::CreateUserRequest,
                         handlers::create_user_handler::CreateUserHandler}};
use di::injectable;


#[injectable]
pub struct UserService {
    create_user_handler: Rc<CreateUserHandler>,
}


impl UserService {
    pub fn save(&self, create_user_request: CreateUserRequest) {
        self.create_user_handler.execute(create_user_request);
    }
}