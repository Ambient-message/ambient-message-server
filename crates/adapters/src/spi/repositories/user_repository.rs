use std::error::Error;

use application::{gateway::repositories::user_repository_abstract::{
    UserRepositoryAbstract,
}, mappers::db_mapper::DbMapper};

use diesel::prelude::*;
use domain::entities::user::User;
use uuid::Uuid;
use crate::spi::db::{db_connection::DbConnection, schema::*, models::UserModel, db_mappers::UserDbMapper};

pub struct UserRepository {
    pub db_connection: DbConnection,
}

impl UserRepositoryAbstract for UserRepository {
    fn save(&self, user: User) -> Result<User, Box<dyn Error>> {

        let mut conn = self
        .db_connection
        .get_pool()
        .get()
        .expect("couldn't get db connection from pool");

        let user_model = UserModel{id : user.id, username: user.username, password: user.password};


        let result = diesel::insert_into(users::table)
        .values(&user_model)
        .returning(UserModel::as_returning())
        .get_result(&mut conn);


        match result {
            Ok(model) => Ok(UserDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }

    }
}
