use std::error::Error;

use domain::chat_entity::ChatEntity;

pub trait ChatRepositoryAbstract {
    fn save(&self, chat: ChatEntity) -> Result<ChatEntity, Box<dyn Error>>;
}