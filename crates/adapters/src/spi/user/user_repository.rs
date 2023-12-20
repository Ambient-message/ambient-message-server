use std::error::Error;

use application::mappers::db_mapper::DbMapper;
use application::repositories::user_repository_abstract::UserRepositoryAbstract;
use diesel::insert_into;
use diesel::prelude::*;
use domain::user::User;

use crate::spi::models::UserModel;
use crate::spi::schema::users::dsl::users;
use crate::spi::user::db_connection::DbConnection;
use crate::spi::user::db_mappers::UserDbMapper;

pub struct UserRepository {
    pub db_connection: DbConnection,
}

impl UserRepositoryAbstract for UserRepository{
    fn save(&self, user: &User) -> Result<User, Box<dyn Error>> {
        let mut conn =
            self.db_connection.get_pool().get().expect("couldn't get db connection from pool");

        let user_model = UserDbMapper::to_db(user.clone());

        let result = insert_into(users)
            .values(user_model)
            .returning(UserModel::as_returning())
            .get_result(&mut conn);

        match result {
            Ok(model) =>{
                let user =  UserDbMapper::to_entity(model);
                Ok(user)
            },
            Err(e) => Err(Box::new(e)),
        }
    }
}