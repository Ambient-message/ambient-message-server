use std::error::Error;
use async_trait::async_trait;

use uuid::Uuid;

use domain::user_entity::UserEntity;

#[async_trait(?Send)]
pub trait UserRepositoryAbstract {
    async fn save(&self, user: &UserEntity) -> Result<(), Box<dyn Error + Send>>;
    async fn find(&self, user_id: Uuid) -> Result<UserEntity, Box<dyn Error + Send>>;
}