use async_trait::async_trait;
use domain::error::ApiError;
use domain::user::User;

use crate::repositories::user_repository_abstract::UserRepositoryAbstract;
use crate::usecases::user::interfaces::AbstractUseCase;

pub struct CreateUser<'r, R>
    where
        R: UserRepositoryAbstract,
{
    user: User,
    repository: &'r R,
}

impl<'r, R> CreateUser<'r, R>
    where
        R: UserRepositoryAbstract,
{
    pub fn new(user: User, repository: &'r R) -> Self {
        Self { user, repository }
    }
}

#[async_trait(? Send)]
impl<'r, R> AbstractUseCase<User> for CreateUser<'r, R>
    where
        R: UserRepositoryAbstract
{
    async fn execute(&self) -> Result<User, ApiError> {
        let user = self.repository.save(&self.user);

        match user {
            Ok(user) => Ok(user),
            Err(e) => Err(ApiError {
                code: 400,
                message: String::from("Cannot create user"),
                error: e,
            }),
        }
    }
}