use uuid::Uuid;

use domain::api_error::ApiError;
use domain::user_entity::UserEntity;

use crate::services::crypto_service_abstract::CryptoServiceAbstract;

pub trait UserRepositoryAbstract {
    async fn save(&self, user: &UserEntity) -> Result<(), ApiError>;
    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<UserEntity>, ApiError>;
    async fn find_by_username(&self, username: impl Into<String>) -> Result<Option<UserEntity>, ApiError>;
    async fn hash_password<C>(&self, user: &UserEntity, crypto: &C) -> Result<UserEntity, ApiError>
    where
        C: CryptoServiceAbstract;
    async fn get_all_users(&self) -> Result<Vec<UserEntity>, ApiError>;
}
