use std::error::Error;

use domain::user_entity::UserEntity;

pub trait UserRepositoryAbstract {
    fn save(&self, user: &UserEntity) -> Result<(), Box<dyn Error>>;
}