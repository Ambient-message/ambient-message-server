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

impl<'a, R, C> AbstractUseCase<String> for AuthUserUseCase<'a, R, C>
    where
        R: UserRepositoryAbstract,
        C: CryptoServiceAbstract,
{
    async fn execute(&self) -> Result<String, ApiError> {
        let user_id = Uuid::parse_str(self.basic.user_id())
            .map_err(|err| ApiError::new(StatusCode::BAD_REQUEST, "Invalid user id", err))?;

        let password = self.basic.password().ok_or(ApiError::new(
            StatusCode::BAD_REQUEST,
            "Invalid user id",
            AppError::EmptyPassword,
        ))?;

        let user = self.repository.find_by_id(user_id).await?;

        //todo perhaps the password should be hashed
        let valid = self
            .hashing
            .verify_password(password, user.password.as_str())
            .await?;

        if valid {
            Ok(self.hashing.generate_jwt(user.id).await?)
        } else {
            Err(ApiError::new(
                StatusCode::UNAUTHORIZED,
                "Wrong password",
                AppError::WrongPassword,
            ))
        }
    }
}
