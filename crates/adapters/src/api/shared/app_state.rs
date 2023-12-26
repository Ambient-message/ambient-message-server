use db::db_connection::DbConnection;
use crate::spi::chat::chat_repository::ChatRepository;
use crate::spi::user::user_repository::UserRepository;

pub struct AppState {
    pub app_name: String,
    pub user_repository: UserRepository,
    pub chat_repository: ChatRepository,
}