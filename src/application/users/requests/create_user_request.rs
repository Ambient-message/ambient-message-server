#[readonly::make]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
}


impl CreateUserRequest {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

