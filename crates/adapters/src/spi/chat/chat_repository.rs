use std::error::Error;
use std::sync::Arc;

use diesel::prelude::*;

use application::mappers::db_mapper::DbMapper;
use application::repositories::chat_repository_abstract::ChatRepositoryAbstract;
use db::db_connection::DbConnection;
use db::models::ChatModel;
use domain::chat_entity::ChatEntity;

use crate::spi::chat::chat_db_mapper::ChatDbMapper;

pub struct ChatRepository {
    pub db_connection: Arc<DbConnection>,
}

impl ChatRepositoryAbstract for ChatRepository {
    fn save(&self, chat: ChatEntity) -> Result<ChatEntity, Box<dyn Error + Send>> {
        let mut conn = self
            .db_connection
            .db_pool
            .get()
            .expect("Couldn't connect to database");

        let chat_model = ChatDbMapper::to_db(chat);

        let result = diesel::insert_into(db::schema::chats::table)
            .values(chat_model)
            .returning(ChatModel::as_returning())
            .get_result(&mut conn);

        match result {
            Ok(chat) => Ok(ChatDbMapper::to_entity(chat)),
            Err(e) => Err(Box::new(e)),
        }
    }
}
