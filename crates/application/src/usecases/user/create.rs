use actix_web::http::StatusCode;
use actix_web_httpauth::extractors::basic::BasicAuth;

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
    basic: BasicAuth,
    repository: &'r R,
crypto: &'c C,
}

impl<'r, 'c, R, C> CreateUserUseCase<'r, 'c, R, C>
    where
        R: UserRepositoryAbstract,
        C: CryptoServiceAbstract
{
    pub fn new(basic: BasicAuth, repository: &'r R, crypto: &'c C) -> Self {
        Self { basic, repository, crypto}
    }
}

impl<'r, 'c, R, C> AbstractUseCase<()> for CreateUserUseCase<'r, 'c, R, C>
    where
        R: UserRepositoryAbstract,
        C: CryptoServiceAbstract
{
    async fn execute(& self) -> Result<(), ApiError> {
        if let Some(_) = self.repository.find_by_username(self.basic.user_id()).await? {
            return Err(ApiError::new(
                StatusCode::BAD_REQUEST,
                "User with this username already exist",
                AppError::UserWithThisUsernameAlreadyExist,
            ));
        }

        let password = self.basic.password().ok_or(ApiError::new(
            StatusCode::BAD_REQUEST,
            "Empty user password",
            AppError::EmptyPassword,
        ))?;

        let password = self.crypto.hash_password(password.into()).await?;

        let user  = UserEntity::new(self.basic.user_id(), password);

        self.repository.save(&user).await
    }
}
