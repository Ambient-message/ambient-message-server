use std::sync::Arc;

use actix_web::http::StatusCode;
use anyhow::Result;
use diesel::{CombineDsl, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use diesel::associations::HasTable;
use uuid::Uuid;

use application::mappers::db_mapper::DbMapper;
use application::repositories::userchat_repository_abstract::UserChatRepositoryAbstract;
use db::db_connection::DbConnection;
use db::models::ChatModel;
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

    fn find(&self, user1_id: Uuid, user2_id: Uuid) -> std::result::Result<Option<Uuid>, ApiError> {
        let mut conn =
            self.db_connection.db_pool.get().map_err(|err| {
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Couldn't connect to database",
                err,
            )
        })?;

        //todo refactor code
        let chats1 =
            db::schema::userchats::dsl::userchats::table()
                .filter(db::schema::userchats::user_model_id.eq(user1_id))
                .inner_join(db::schema::chats::table)
                .select(db::schema::chats::all_columns);


        let chats2 =
            db::schema::userchats::dsl::userchats::table()
                .filter(db::schema::userchats::user_model_id.eq(user2_id))
                .inner_join(db::schema::chats::table)
                .select(db::schema::chats::all_columns);

        let common_chat = chats1
            .intersect(chats2)
            .get_result::<ChatModel>(&mut conn)
            .optional()
        .map_err(|err| ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Couldn't find chat",
            err,
        ))?;
        //
        // for i in common_chat {
        //     println!("{}", i.id);
        // }

        Ok(common_chat.map(|c|
            {
                println!("Find chat {}", c.id);
                c.id
            }))
    }
}
