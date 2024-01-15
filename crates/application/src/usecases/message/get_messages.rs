use actix_web::http::StatusCode;
use uuid::Uuid;
use domain::api_error::ApiError;
use domain::message_entity::MessageEntity;
use domain::user_entity::UserEntity;
use crate::repositories::message_repository_abstract::MessageRepositoryAbstract;
use crate::repositories::user_repository_abstract::UserRepositoryAbstract;
use crate::services::crypto_service_abstract::CryptoServiceAbstract;
use crate::shared::app_error::AppError;
use crate::usecases::interfaces::AbstractUseCase;

pub struct GetMessagesUseCase<'r, R>
    where
        R: MessageRepositoryAbstract,
{
    chat_id: Uuid,
    repository: &'r R,
}

impl<'r, R> GetMessagesUseCase<'r, R>
    where
        R: MessageRepositoryAbstract,
{
    pub fn new(chat_id: Uuid, repository: &'r R) -> Self {
        Self { chat_id, repository}
    }
}

impl<'r, R> AbstractUseCase<Vec<MessageEntity>> for GetMessagesUseCase<'r, R>
    where
        R: MessageRepositoryAbstract,
{
    async fn execute(&self) -> Result<Vec<MessageEntity>, ApiError> {
        self.repository.get_messages(&self.chat_id).await
    }
}