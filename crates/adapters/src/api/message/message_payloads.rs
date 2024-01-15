use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct MessagePayload {
    pub chat_id: Uuid,
    pub text: String,
}