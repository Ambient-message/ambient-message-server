use domain::entities::{error::ApiError, user::User};
use uuid::Uuid;

use crate::{
    gateway::repositories::user_repository_abstract::UserRepositoryAbstract,
    utils::error_handling_utils::ErrorHandlingUtils,
};

pub struct Request {
    pub username: String,
    pub password: String,
}

pub struct Response {
    pub user: User,
}

pub struct CreateUser;

impl CreateUser {
    pub fn new() -> Self {
        Self
    }

    pub fn exec(&self, req: Request) -> Result<Response, ApiError> {
        let user = User::new(Uuid::new_v4(), req.username, req.password);

        Ok(Response { user })
    }
}
