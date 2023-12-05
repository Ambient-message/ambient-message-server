use crate::domain::user::user::User;
use mockall::predicate::*;
use mockall::*;

#[automock]
pub trait UserRepositoryAbstract {
    fn by_id(&self, id: i32) -> Result<User, String>;
    fn save(&self, client: User);
    fn next_identity(&self) -> String;
    fn all(&self) -> Vec<User>;
}