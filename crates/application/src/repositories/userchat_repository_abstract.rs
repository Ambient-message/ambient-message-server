use std::error::Error;

use domain::user_chat_entity::UserChatEntity;

pub trait UserChatRepositoryAbstract {
    fn save(&self, user: &UserChatEntity) -> Result<(), Box<dyn Error + Send>>;
}