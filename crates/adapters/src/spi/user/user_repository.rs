use std::error::Error;

use diesel::prelude::*;

use application::mappers::db_mapper::DbMapper;
use application::repositories::user_repository_abstract::UserRepositoryAbstract;
use db::db_connection::DbConnection;
use db::schema::users::dsl::users;
use domain::user_entity::UserEntity;

use crate::spi::user::db_mappers::UserDbMapper;

pub struct UserRepository {
    pub db_connection: DbConnection,
}

impl UserRepositoryAbstract for UserRepository {
    fn save(&self, user: &UserEntity) -> Result<(), Box<dyn Error>> {
        let mut conn =
            self.db_connection.db_pool.get().expect("Couldn't connect to database");

        let user_model = UserDbMapper::to_db(user.clone());

        let result = diesel::insert_into(users)
            .values(user_model)
            .execute(&mut conn);

        match result {
            Ok(_) => {
                Ok(())
            }
            Err(e) => Err(Box::new(e)),
        }
    }
}