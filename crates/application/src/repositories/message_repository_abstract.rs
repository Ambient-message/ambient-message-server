use uuid::Uuid;
use domain::api_error::ApiError;
use domain::message_entity::MessageEntity;

pub trait MessageRepositoryAbstract {
    async fn save(&self, message: &MessageEntity) -> Result<(), ApiError>;
    async fn get_messages(&self, chat_id: &Uuid) -> Result<Vec<MessageEntity>, ApiError>;
}