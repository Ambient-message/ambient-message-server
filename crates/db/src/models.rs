use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

use crate::schema::users;
use crate::schema::chats;
use crate::schema::userchats;

#[derive(Debug, Insertable, Queryable, Identifiable, Selectable)]
#[diesel(table_name = users)]
pub struct UserModel {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Insertable, Queryable, Identifiable, Selectable)]
#[diesel(table_name = chats)]
pub struct ChatModel{
    pub id: Uuid
}

#[derive(Debug, Insertable, Queryable, Identifiable, Selectable, Associations)]
#[diesel(belongs_to(UserModel))]
#[diesel(belongs_to(ChatModel))]
#[diesel(table_name = userchats)]
pub struct UserChatModel {
    pub id: Uuid,
    pub user_model_id: Uuid,
    pub chat_model_id: Uuid,
}