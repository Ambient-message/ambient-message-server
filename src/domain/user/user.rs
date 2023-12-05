use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new(id: i32, username: &str, password: &str) -> Self {
        Self {
            id,
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}
