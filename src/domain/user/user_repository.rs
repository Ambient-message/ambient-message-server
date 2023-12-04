use crate::domain::user::user::User;
use mockall::predicate::*;
use mockall::*;

#[automock]
pub trait UserRepository {
    fn by_id(&self, id: &str) -> Result<User, String>;
    fn save(&self, client: User);
    fn next_identity(&self) -> String;
    fn all(&self) -> Vec<User>;
}