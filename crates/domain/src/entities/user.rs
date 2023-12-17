use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new<N>(id: Uuid, name: N, password: N) -> Self
    where
        N: Into<String>,
    {
        Self {
            id,
            username: name.into(),
            password: password.into(),
        }
    }
}
