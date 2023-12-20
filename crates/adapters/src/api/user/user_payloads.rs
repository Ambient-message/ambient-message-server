use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct UserPayload {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}