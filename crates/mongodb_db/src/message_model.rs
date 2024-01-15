use serde::{Deserialize, Serialize};
use uuid::Uuid;
use bson::serde_helpers::uuid_1_as_binary;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageModel {
    #[serde(with = "uuid_1_as_binary")]
    pub _id: Uuid,
    #[serde(with = "uuid_1_as_binary")]
    pub sender_id: Uuid,
    #[serde(with = "uuid_1_as_binary")]
    pub chat_id: Uuid,
    pub text: String,
}

impl MessageModel {
    fn new(sender_id: Uuid, chat_id: Uuid, text: String) -> Self {
        Self {
            _id: Uuid::new_v4(),
            sender_id,
            chat_id,
            text,
        }
    }
}