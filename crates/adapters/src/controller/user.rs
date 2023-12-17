use application::gateway::repositories::user_repository_abstract::Repo;
use application::usecases::user::create::{CreateUser, Request};
use domain::entities::user::User;

pub struct Controller<'d, D>
where
    D: Repo,
{
    db: &'d D,
}

impl<'d, D> Controller<'d, D>
where
    D: Repo,
{
    pub fn new(db: &'d D) -> Self {
        Self { db }
    }

    pub fn create_user<T>(&self, username: T, password: T) -> User
    where
        T: Into<String>,
    {
        let req = Request {
            username: username.into(),
            password: password.into(),
        };
        let interactor = CreateUser::new(self.db);
        let res = interactor.exec(req);
        if let Ok(r) = res {
            return r.user;
        }
        panic!("An error on create user!!")
    }
}
