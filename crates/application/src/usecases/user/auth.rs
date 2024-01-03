use actix_web_httpauth::extractors::basic::BasicAuth;
use async_trait::async_trait;
use uuid::Uuid;

use domain::api_error::ApiError;

use crate::repositories::user_repository_abstract::UserRepositoryAbstract;
use crate::services::crypto_service_abstract::CryptoServiceAbstract;
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

#[async_trait(? Send)]
impl<'a, R, C> AbstractUseCase<Option<String>> for AuthUserUseCase<'a, R, C>
where
    R: UserRepositoryAbstract,
    C: CryptoServiceAbstract,
{
    async fn execute(&self) -> Result<Option<String>, ApiError> {
        let user_id = match Uuid::parse_str(self.basic.user_id()) {
            Ok(uuid) => uuid,
            Err(e) => {
                return Err(ApiError {
                    code: 401,
                    message: String::from("Invalid user id"),
                    error: Box::new(e),
                })
            }
        };

        let password = match self.basic.password() {
            Some(p) => p,
            None => return Ok(None),
        };

        let user = match self.repository.find(user_id).await {
            Ok(user) => user,
            Err(e) => {
                return Err(ApiError {
                    code: 401,
                    message: String::from("User dosn't ecxist"),
                    error: e,
                })
            }
        };

        //todo perhaps the password should be hashed
        let valid = self
            .hashing
            .verify_password(password, user.password.as_str())
            .await?;

        if valid {
            let token = self.hashing.generate_jwt(user.id).await?;
            Ok(Some(token))
        } else {
            Ok(None)
        }
    }
}
