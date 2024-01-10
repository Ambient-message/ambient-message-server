use domain::api_error::ApiError;
use domain::chat_entity::ChatEntity;

pub trait ChatRepositoryAbstract {
    fn save(&self, chat: ChatEntity) -> Result<ChatEntity, ApiError>;
}
