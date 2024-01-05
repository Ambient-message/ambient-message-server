use std::error::Error;
use std::sync::Arc;

use diesel::prelude::*;
use uuid::Uuid;

use application::mappers::db_mapper::DbMapper;
use application::repositories::user_repository_abstract::UserRepositoryAbstract;
use db::db_connection::DbConnection;
use db::schema::users::dsl::users;
use domain::user_entity::UserEntity;

use crate::spi::user::chat_db_mapper::UserDbMapper;

pub struct UserRepository {
    pub db_connection: Arc<DbConnection>,
}

impl UserRepositoryAbstract for UserRepository {
    async fn save(&self, user: &UserEntity) -> Result<(), Box<dyn Error + Send>> {
        let mut conn = self
            .db_connection
            .db_pool
            .get()
            .expect("Couldn't connect to database");

        let user_model = UserDbMapper::to_db(user.clone());

        let result = diesel::insert_into(users)
            .values(user_model)
            .execute(&mut conn);

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn find(&self, user_id: Uuid) -> Result<UserEntity, Box<dyn Error + Send>> {
        use diesel::prelude::*;

        let mut conn = self
            .db_connection
            .db_pool
            .get()
            .expect("Couldn't connect to database");

        let result = db::schema::users::table.find(user_id).first(&mut conn);


        match result {
            Ok(user) => Ok(UserDbMapper::to_entity(user)),
            Err(e) => Err(Box::new(e)),
        }
    }
}