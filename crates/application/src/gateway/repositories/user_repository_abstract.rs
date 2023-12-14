use domain::entities::user::User;
use std::error::Error;

pub struct Record {
    pub user: User,
}

pub trait UserRepositoryAbstract {
    fn save(&self, record: Record) -> Result<User, Box<dyn Error>>;
}
