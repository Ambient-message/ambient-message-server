use actix_web::http::StatusCode;

use domain::api_error::ApiError;
use domain::user_entity::UserEntity;

use crate::repositories::user_repository_abstract::UserRepositoryAbstract;
use crate::shared::app_error::AppError;
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

impl<'r, R> AbstractUseCase<()> for CreateUserUseCase<'r, R>
    where
        R: UserRepositoryAbstract,
{
    async fn execute(&self) -> Result<(), ApiError> {
        if let Ok(_) = self.repository.find_by_username(&self.user.username).await {
            return Err(ApiError::new(
                StatusCode::BAD_REQUEST,
                "User with this username already exist",
                AppError::UserWithThisUsernameAlreadyExist,
            ));
        }

        self.repository.save(&self.user).await
    }
}
