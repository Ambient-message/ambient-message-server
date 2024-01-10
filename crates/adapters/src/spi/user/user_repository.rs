use std::sync::Arc;

use actix_web::http::StatusCode;
use anyhow::Result;
use diesel::prelude::*;
use uuid::Uuid;

use application::mappers::db_mapper::DbMapper;
use application::repositories::user_repository_abstract::UserRepositoryAbstract;
use application::services::crypto_service_abstract::CryptoServiceAbstract;
use application::shared::app_error::AppError;
use db::db_connection::DbConnection;
use db::models::UserModel;
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

    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<UserEntity>, ApiError> {
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
            .map(UserDbMapper::to_entity);

        Ok(result)
    }

    async fn find_by_username(&self, username: impl Into<String>) -> Result<Option<UserEntity>, ApiError> {
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
            .filter(db::schema::users::username.eq(username.into()))
            .first::<UserModel>(&mut conn)
            .optional()
            .map_err(|err| {
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Can't find user", err)
            })?
            .map(UserDbMapper::to_entity);

        Ok(result)
    }

    async fn hash_password<C>(&self, user: &UserEntity, crypto: &C) -> Result<UserEntity, ApiError>
    where C: CryptoServiceAbstract{
        let hashed_password = crypto.hash_password(&user.password).await?;
        Ok(user.change_password(hashed_password))
    }
}
