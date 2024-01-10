use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UserChatEntity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub chat_id: Uuid,
}

impl UserChatEntity {
    pub fn new(user_id: Uuid, chat_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            chat_id,
        }
    }
}
