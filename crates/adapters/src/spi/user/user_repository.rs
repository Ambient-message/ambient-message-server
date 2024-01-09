use std::sync::Arc;

use actix_web::http::StatusCode;
use anyhow::Result;
use diesel::prelude::*;
use uuid::Uuid;

use application::mappers::db_mapper::DbMapper;
use application::repositories::user_repository_abstract::UserRepositoryAbstract;
use application::shared::app_error::AppError;
use db::db_connection::DbConnection;
use db::schema::users::dsl::users;
use domain::api_error::ApiError;
use domain::user_entity::UserEntity;

use crate::spi::user::chat_db_mapper::UserDbMapper;

pub struct UserRepository {
    pub db_connection: Arc<DbConnection>,
}

impl UserRepositoryAbstract for UserRepository {
    async fn save(&self, user: &UserEntity) -> Result<(), ApiError> {
        let mut conn =
            self.db_connection.db_pool.get().map_err(|err| {
                ApiError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Couldn't connect to database",
                    err,
                )
            })?;

        let user_model = UserDbMapper::to_db(user.clone());

        diesel::insert_into(users)
            .values(user_model)
            .execute(&mut conn)
            .map(|_| ())
            .map_err(|err| {
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Can't insert user", err)
            })
    }

    async fn find_by_id(&self, user_id: Uuid) -> Result<UserEntity, ApiError> {
        use diesel::prelude::*;

        let mut conn = self
            .db_connection
            .db_pool
            .get()
            .map_err(|err| {
                ApiError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Couldn't connect to database",
                    err,
                )
            })?;

        let result = db::schema::users::table
            .find(user_id)
            .first(&mut conn)
            .optional()
            .map_err(|err| {
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Can't find user", err)
            })?
            .ok_or(ApiError::new(
                StatusCode::NOT_FOUND,
                "Can't find user",
                AppError::UserNotFound,
            ))?;

        Ok(UserDbMapper::to_entity(result))
    }
}
