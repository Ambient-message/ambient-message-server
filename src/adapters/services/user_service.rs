use std::rc::Rc;

use crate::{domain::user::{user_repository_abstract::UserRepositoryAbstract, user::User},
    application::users::{requests::create_user_request::CreateUserRequest,
        handlers::create_user_handler::{self, CreateUserHandler}}, adapters::spi::db::schema::users::password};
use di::injectable;


#[injectable]
pub struct UserService{
    create_user_handler : Rc<CreateUserHandler>
}


impl UserService {
    pub fn save(&self, create_user_request : CreateUserRequest){
        self.create_user_handler.execute(create_user_request);
    }
}