use uuid::Uuid;


#[derive(Debug)]
pub struct UserPayload {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}