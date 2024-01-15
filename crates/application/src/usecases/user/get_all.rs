use actix_web::http::StatusCode;
use uuid::Uuid;

use domain::api_error::ApiError;
use domain::user_entity::UserEntity;

use crate::repositories::user_repository_abstract::UserRepositoryAbstract;
use crate::shared::app_error::AppError;
use crate::usecases::interfaces::AbstractUseCase;

pub struct GetAllUsersUseCase<'r, R>
    where
        R: UserRepositoryAbstract,
{
    repository: &'r R,
}

impl<'r, R> GetAllUsersUseCase<'r, R>
    where
        R: UserRepositoryAbstract,
{
    pub fn new(repository: &'r R) -> Self {
        Self {
            repository,
        }
    }
}

impl<'r, R> AbstractUseCase<Vec<UserEntity>> for GetAllUsersUseCase<'r, R>
    where
        R: UserRepositoryAbstract,
{
    async fn execute(&self) -> Result<Vec<UserEntity>, ApiError> {
        self.repository.get_all_users().await
    }
}
