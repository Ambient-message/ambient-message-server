use std::sync::Arc;

use actix_web::http::StatusCode;
use anyhow::Result;
use diesel::RunQueryDsl;

use application::mappers::db_mapper::DbMapper;
use application::repositories::userchat_repository_abstract::UserChatRepositoryAbstract;
use db::db_connection::DbConnection;
use domain::api_error::ApiError;
use domain::user_chat_entity::UserChatEntity;

use crate::spi::user_chat::user_chat_db_mapper::UserChatDbMapper;

pub struct UserChatRepository {
    pub db_connection: Arc<DbConnection>,
}

impl UserChatRepositoryAbstract for UserChatRepository {
    fn save(&self, user_chat: &UserChatEntity) -> Result<(), ApiError> {
        let mut conn = self.db_connection.db_pool.get().map_err(|err| {
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Couldn't connect to database",
                err,
            )
        })?;

        let user_chat_model = UserChatDbMapper::to_db(user_chat.clone());

        diesel::insert_into(db::schema::userchats::table)
            .values(user_chat_model)
            .execute(&mut conn)
            .map_err(|err| {
                ApiError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Con't insert userchat",
                    err,
                )
            })?;

        Ok(())
    }
}
