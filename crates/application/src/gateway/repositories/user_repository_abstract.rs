use domain::entities::user::User;
use thiserror;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("Area of life repository connection problem")]
    Connection,
}

pub struct Record {
    pub user: User,
}

pub trait Repo {
    fn save(&self, record: Record) -> Result<User, SaveError>;
}
