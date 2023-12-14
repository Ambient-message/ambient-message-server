use domain::entities::user::User;
use uuid::Uuid;


pub struct Request {
    pub username: String,
    pub password : String,
}

pub struct Response {
    pub user : User
}

pub struct CreateUser;

impl CreateUser {
    pub fn new() -> Self {
        Self
    }

    pub fn exec(&self, req: Request) -> Result<Response, ()> {

        let user = User::new(Uuid::new_v4(), req.username, req.password);

        Ok(Response{user})
    }
}