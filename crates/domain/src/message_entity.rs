use uuid::Uuid;

pub struct MessageEntity {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub chat_id: Uuid,
    pub text: String
}