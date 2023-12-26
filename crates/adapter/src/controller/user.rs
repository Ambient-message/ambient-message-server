use application::gateway::usecase::user::create::{CreateUser, Request, Response};

pub struct Controller;

impl Controller {
    pub fn create_user<N>(&self, name: N) -> Response
    where
        N: Into<String>,
    {
        let req = Request { name: name.into() };
        let interactor = CreateUser::new();
        interactor.exec(req).unwrap()
    }
}
