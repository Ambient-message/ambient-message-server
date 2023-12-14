use domain::entities::user::User;
use std::error::Error;

pub trait UserRepositoryAbstract {
    fn save(&self, user: User) -> Result<User, Box<dyn Error>>;
}
