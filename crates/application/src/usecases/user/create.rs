use actix_web::http::StatusCode;

use domain::api_error::ApiError;
use domain::user_entity::UserEntity;

use crate::repositories::user_repository_abstract::UserRepositoryAbstract;
use crate::services::crypto_service_abstract::CryptoServiceAbstract;
use crate::shared::app_error::AppError;
use crate::usecases::interfaces::AbstractUseCase;

pub struct CreateUserUseCase<'r, 'c, R, C>
    where
        R: UserRepositoryAbstract,
C: CryptoServiceAbstract
{
    user: UserEntity,
    repository: &'r R,
crypto: &'c C,
}

impl<'r, 'c, R, C> CreateUserUseCase<'r, 'c, R, C>
    where
        R: UserRepositoryAbstract,
        C: CryptoServiceAbstract
{
    pub fn new(user: UserEntity, repository: &'r R, crypto: &'c C) -> Self {
        Self { user, repository, crypto}
    }
}

impl<'r, 'c, R, C> AbstractUseCase<()> for CreateUserUseCase<'r, 'c, R, C>
    where
        R: UserRepositoryAbstract,
        C: CryptoServiceAbstract
{
    async fn execute(& self) -> Result<(), ApiError> {
        if let Some(_) = self.repository.find_by_username(&self.user.username).await? {
            return Err(ApiError::new(
                StatusCode::BAD_REQUEST,
                "User with this username already exist",
                AppError::UserWithThisUsernameAlreadyExist,
            ));
        }
        let user = self.repository.hash_password(&self.user, self.crypto).await?;
        self.repository.save(&user).await
    }
}
