use uuid::Uuid;

use domain::api_error::ApiError;
use domain::user_entity::UserEntity;

use crate::repositories::user_repository_abstract::UserRepositoryAbstract;
use crate::usecases::interfaces::AbstractUseCase;

pub struct FindUserUseCase<'r, R>
    where
        R: UserRepositoryAbstract,
{
    user_id: Uuid,
    repository: &'r R,
}

impl<'r, R> FindUserUseCase<'r, R>
    where
        R: UserRepositoryAbstract,
{
    pub fn new(user_id: Uuid, repository: &'r R) -> Self {
        Self { user_id, repository }
    }
}

impl<'r, R> AbstractUseCase<UserEntity> for FindUserUseCase<'r, R>
    where
        R: UserRepositoryAbstract
{
    async fn execute(&self) -> Result<UserEntity, ApiError> {
        let result = self.repository.find(self.user_id).await;

        match result {
            Ok(user) => Ok(user),
            Err(e) => Err(ApiError {
                code: 400,
                message: String::from("Cannot create user"),
                error: e,
            }),
        }
    }
}