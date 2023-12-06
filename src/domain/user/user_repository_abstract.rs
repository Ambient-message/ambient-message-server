use crate::domain::user::user::User;
use di::{inject, injectable};
use mockall::predicate::*;
use mockall::*;
use async_trait::async_trait;
use std::error::Error;

#[automock]
pub trait UserRepositoryAbstract {
    fn save(&self, user : User) -> Result<User, Box<dyn Error>>;
}