use uuid::Uuid;

use domain::api_error::ApiError;
use domain::user_entity::UserEntity;

pub trait UserRepositoryAbstract {
    async fn save(&self, user: &UserEntity) -> Result<(), ApiError>;
    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<UserEntity>, ApiError>;
    async fn find_by_username(&self, username: impl Into<String>) -> Result<Option<UserEntity>, ApiError>;
}
