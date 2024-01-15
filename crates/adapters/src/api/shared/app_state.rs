use application::services::crypto_service_abstract::CryptoServiceAbstract;
use crate::services::crypto::CryptoService;
use crate::spi::chat::chat_repository::ChatRepository;
use crate::spi::message::message_repository::MessageRepository;
use crate::spi::user::user_repository::UserRepository;
use crate::spi::user_chat::user_chat_repository::UserChatRepository;

pub struct AppState {
    pub app_name: String,
    pub user_repository: UserRepository,
    pub chat_repository: ChatRepository,
    pub user_chat_repository: UserChatRepository,
    pub message_repository: MessageRepository,
    pub crypto_services: CryptoService,
}
