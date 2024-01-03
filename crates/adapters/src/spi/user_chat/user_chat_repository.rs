use std::error::Error;
use std::sync::Arc;

use diesel::RunQueryDsl;

use application::mappers::db_mapper::DbMapper;
use application::repositories::userchat_repository_abstract::UserChatRepositoryAbstract;
use db::db_connection::DbConnection;
use domain::user_chat_entity::UserChatEntity;

use crate::spi::user_chat::user_chat_db_mapper::UserChatDbMapper;

pub struct UserChatRepository {
    pub db_connection: Arc<DbConnection>,
}

impl UserChatRepositoryAbstract for UserChatRepository {
    fn save(&self, user_chat: &UserChatEntity) -> Result<(), Box<dyn Error + Send>> {
        let mut conn = self
            .db_connection
            .db_pool
            .get()
            .expect("Couldn't connect to database");

        let user_chat_model = UserChatDbMapper::to_db(user_chat.clone());

        let result = diesel::insert_into(db::schema::userchats::table)
            .values(user_chat_model)
            .execute(&mut conn);

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
