use std::error::Error;

use uuid::Uuid;

use domain::user_entity::UserEntity;

pub trait UserRepositoryAbstract {
    async fn save(&self, user: &UserEntity) -> Result<(), Box<dyn Error + Send>>;
    async fn find(&self, user_id: Uuid) -> Result<UserEntity, Box<dyn Error + Send>>;
}