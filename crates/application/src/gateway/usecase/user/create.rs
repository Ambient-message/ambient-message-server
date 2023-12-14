use domain::entity::user::User;

pub struct Request {
    pub name: String,
}

pub struct Response {
    pub user: User,
}

pub struct CreateUser;

impl CreateUser {
    pub fn new() -> Self {
        Self
    }

    pub fn exec(&self, reg: Request) -> Result<Response, ()> {
        let user = User::new(1, reg.name);
        Ok(Response { user })
    }
}
