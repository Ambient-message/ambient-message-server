use std::error::Error;
use domain::user::User;

pub trait UserRepositoryAbstract {
    fn save(&self, user: &User) -> Result<(), Box<dyn Error>>;
}