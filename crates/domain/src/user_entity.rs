use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UserEntity {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}

impl UserEntity {
    pub fn new<N>(name: N, password: N) -> Self
        where
            N: Into<String>,
    {
        Self {
            id: Uuid::new_v4(),
            username: name.into(),
            password: password.into(),
        }
    }

    pub fn change_password(&self, password: impl Into<String>) -> Self {
        Self {
            id: self.id,
            username: self.username.clone(),
            password: password.into(),
        }
    }
}
