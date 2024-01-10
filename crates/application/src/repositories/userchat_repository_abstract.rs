use domain::api_error::ApiError;
use domain::user_chat_entity::UserChatEntity;

pub trait UserChatRepositoryAbstract {
    fn save(&self, user: &UserChatEntity) -> Result<(), ApiError>;
}
