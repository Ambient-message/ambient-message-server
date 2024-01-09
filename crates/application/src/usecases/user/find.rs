use actix_web::http::StatusCode;
use uuid::Uuid;

use domain::api_error::ApiError;
use domain::user_entity::UserEntity;

use crate::repositories::user_repository_abstract::UserRepositoryAbstract;
use crate::shared::app_error::AppError;
use crate::usecases::interfaces::AbstractUseCase;

pub struct FindUserByIDUseCase<'r, R>
    where
        R: UserRepositoryAbstract,
{
    user_id: Uuid,
    repository: &'r R,
}

impl<'r, R> FindUserByIDUseCase<'r, R>
    where
        R: UserRepositoryAbstract,
{
    pub fn new(user_id: Uuid, repository: &'r R) -> Self {
        Self {
            user_id,
            repository,
        }
    }
}

impl<'r, R> AbstractUseCase<UserEntity> for FindUserByIDUseCase<'r, R>
    where
        R: UserRepositoryAbstract,
{
    async fn execute(&self) -> Result<UserEntity, ApiError> {
        self.repository.find_by_id(self.user_id).await?
            .ok_or(ApiError::new(
            StatusCode::BAD_REQUEST,
            "User with this id doesn't not exist",
            AppError::UserNotFound,
        ))
    }
}
