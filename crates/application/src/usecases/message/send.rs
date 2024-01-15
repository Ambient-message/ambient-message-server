use actix_web::http::StatusCode;
use domain::api_error::ApiError;
use domain::message_entity::MessageEntity;
use domain::user_entity::UserEntity;
use crate::repositories::message_repository_abstract::MessageRepositoryAbstract;
use crate::repositories::user_repository_abstract::UserRepositoryAbstract;
use crate::services::crypto_service_abstract::CryptoServiceAbstract;
use crate::shared::app_error::AppError;
use crate::usecases::interfaces::AbstractUseCase;

pub struct SendMessageUseCase<'r, R>
    where
        R: MessageRepositoryAbstract,
{
    message: MessageEntity,
    repository: &'r R,
}

impl<'r, R> SendMessageUseCase<'r, R>
    where
        R: MessageRepositoryAbstract,
{
    pub fn new(message: MessageEntity, repository: &'r R) -> Self {
        Self { message, repository}
    }
}

impl<'r, R> AbstractUseCase<()> for SendMessageUseCase<'r, R>
    where
        R: MessageRepositoryAbstract,
{
    async fn execute(&self) -> Result<(), ApiError> {
        self.repository.save(&self.message).await
    }
}
