use super::controller::user::Controller;
use application::gateway::repositories::user_repository_abstract::Repo;
use domain::entities::user::User;

pub struct Api<D>
where
    D: Repo,
{
    //TODO Arc
    db: D,
}

impl<D> Api<D>
where
    D: Repo,
{
    pub fn new(db: D) -> Self {
        Self { db }
    }

    fn user_controller(&self) -> Controller<D> {
        Controller::new(&self.db)
    }

    pub fn create_user<T>(self, username: T, password: T) -> User
    where
        T: Into<String>,
    {
        self.user_controller().create_user(username, password)
    }
}
