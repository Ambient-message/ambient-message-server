use crate::spi::user::user_repository::UserRepository;

pub struct AppState {
    pub app_name: String,
    pub user_repository: UserRepository,
}