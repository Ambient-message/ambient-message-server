use uuid::Uuid;
use domain::api_error::ApiError;
use domain::chat_entity::ChatEntity;
use domain::user_chat_entity::UserChatEntity;

pub trait UserChatRepositoryAbstract {
    fn save(&self, user: &UserChatEntity) -> Result<(), ApiError>;
    fn find(&self, user1_id: Uuid, user2_id: Uuid) -> Result<Option<Uuid>, ApiError>;
}
