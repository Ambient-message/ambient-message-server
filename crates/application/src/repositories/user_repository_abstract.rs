use std::error::Error;

use domain::user_entity::UserEntity;
use uuid::Uuid;

pub trait UserRepositoryAbstract {
    fn save(&self, user: &UserEntity) -> Result<(), Box<dyn Error>>;
    fn find(&self, user_id: Uuid) -> Result<UserEntity, Box<dyn Error>>;
}