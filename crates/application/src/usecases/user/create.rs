use async_trait::async_trait;

use domain::error::ApiError;
use domain::user_entity::UserEntity;

use crate::repositories::user_repository_abstract::UserRepositoryAbstract;
use crate::usecases::interfaces::AbstractUseCase;

pub struct CreateUserUseCase<'r, R>
    where
        R: UserRepositoryAbstract,
{
    user: UserEntity,
    repository: &'r R,
}

impl<'r, R> CreateUserUseCase<'r, R>
    where
        R: UserRepositoryAbstract,
{
    pub fn new(user: UserEntity, repository: &'r R) -> Self {
        Self { user, repository }
    }
}

#[async_trait(? Send)]
impl<'r, R> AbstractUseCase<()> for CreateUserUseCase<'r, R>
    where
        R: UserRepositoryAbstract
{
    async fn execute(&self) -> Result<(), ApiError> {
        let result = self.repository.save(&self.user);

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(ApiError {
                code: 400,
                message: String::from("Cannot create user"),
                error: e,
            }),
        }
    }
}