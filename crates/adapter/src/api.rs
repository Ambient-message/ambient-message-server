use crate::controller::user::Controller;
use application::gateway::usecase::user::create::Response;

pub struct Api;

impl Api {
    fn user_controller(&self) -> Controller {
        Controller
    }

    pub fn create_user<N>(&self, name: N) -> Response
    where
        N: Into<String>,
    {
        self.user_controller().create_user(name)
    }
}
