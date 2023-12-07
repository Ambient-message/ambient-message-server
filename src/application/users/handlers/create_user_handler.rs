use std::fmt::Error;
use crate::application::users::requests::create_user_request::CreateUserRequest;
use crate::domain::user::user::User;
use crate::domain::user::user_repository_abstract::UserRepositoryAbstract;
use di::*;
use uuid::Uuid;
use std::rc::Rc;

#[injectable]
pub struct CreateUserHandler {
    user_repository: Rc<dyn UserRepositoryAbstract>,
}


impl CreateUserHandler {
    pub fn new(user_repository: Rc<dyn UserRepositoryAbstract>) -> CreateUserHandler {
        CreateUserHandler { user_repository }
    }

    pub fn execute(&self, request: CreateUserRequest) -> Result<User, Error> {
        let id = Uuid::new_v4();

        let user = User::new(
            id,
            request.username.as_str(),
            request.password.as_str(),
        );

        Ok(self.user_repository.save(user).expect("User does`t saved"))
    }
}