use domain::entities::user::User;
use thiserror::Error;
use uuid::Uuid;

use crate::gateway::repositories::user_repository_abstract::{Record, Repo, SaveError};

pub struct Request {
    pub username: String,
    pub password: String,
}

pub struct Response {
    pub user: User,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}", SaveError::Connection)]
    Repo,
}

impl From<SaveError> for Error {
    fn from(e: SaveError) -> Self {
        match e {
            SaveError::Connection => Self::Repo,
        }
    }
}

pub struct CreateUser<'r, R>
    where
        R: Repo,
{
    repo: &'r R,
}

impl<'r, R> CreateUser<'r, R>
    where
        R: Repo,
{
    pub fn new(repo: &'r R) -> Self {
        Self { repo }
    }

    pub fn exec(&self, req: Request) -> Result<Response, Error> {
        let user = User::new(Uuid::new_v4(), req.username, req.password);
        let rep = Record { user: user.clone() };
        let _ = self.repo.save(rep);
        Ok(Response { user })
    }
}
