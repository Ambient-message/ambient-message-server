use crate::spi::chat::chat_repository::ChatRepository;
use crate::spi::user::user_repository::UserRepository;
use crate::spi::user_chat::user_chat_repository::UserChatRepository;

pub struct AppState {
    pub app_name: String,
    pub user_repository: UserRepository,
    pub chat_repository: ChatRepository,
    pub user_chat_repository: UserChatRepository,
}
