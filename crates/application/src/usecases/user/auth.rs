use actix_web::http::StatusCode;
use actix_web_httpauth::extractors::basic::BasicAuth;
use uuid::Uuid;

use domain::api_error::ApiError;

use crate::repositories::user_repository_abstract::UserRepositoryAbstract;
use crate::services::crypto_service_abstract::CryptoServiceAbstract;
use crate::shared::app_error::AppError;
use crate::usecases::interfaces::AbstractUseCase;

pub struct AuthUserUseCase<'a, R, C>
    where
        R: UserRepositoryAbstract,
        C: CryptoServiceAbstract,
{
    basic: BasicAuth,
    repository: &'a R,
    hashing: &'a C,
}

impl<'a, R, C> AuthUserUseCase<'a, R, C>
    where
        R: UserRepositoryAbstract,
        C: CryptoServiceAbstract,
{
    pub fn new(basic: BasicAuth, repository: &'a R, hashing: &'a C) -> Self {
        Self {
            basic,
            repository,
            hashing,
        }
    }
}

impl<'a, R, C> AbstractUseCase<(Uuid, String)> for AuthUserUseCase<'a, R, C>
    where
        R: UserRepositoryAbstract,
        C: CryptoServiceAbstract,
{
    async fn execute(&self) -> Result<(Uuid, String), ApiError> {
        let username = self.basic.user_id();

        let password = self.basic.password().ok_or(ApiError::new(
            StatusCode::BAD_REQUEST,
            "Empty user password",
            AppError::EmptyPassword,
        ))?;

        let user = self.repository.find_by_username(username).await?
            .ok_or(ApiError::new(
                StatusCode::BAD_REQUEST,
                "User with this username doesn't not exist",
                AppError::UserNotExist,
            ))?;

        let valid = self
            .hashing
            .verify_password(password, user.password.as_str())
            .await?;

        if valid {
            Ok((user.id, self.hashing.generate_jwt(user.id).await?))
        } else {
            Err(ApiError::new(
                StatusCode::UNAUTHORIZED,
                "Wrong password",
                AppError::WrongPassword,
            ))
        }
    }
}
