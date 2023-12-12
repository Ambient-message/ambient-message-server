use crate::domain::user::user::User;

use mockall::predicate::*;
use mockall::*;

use std::error::Error;

#[automock]
pub trait UserRepositoryAbstract {
    fn save(&self, user : User) -> Result<User, Box<dyn Error>>;
}