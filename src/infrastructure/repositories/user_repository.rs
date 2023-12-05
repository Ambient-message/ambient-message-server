use async_trait::async_trait;
use diesel::RunQueryDsl;
use std::error::Error;
use crate::adapters::spi::db::db_connection;
use crate::adapters::spi::db::schema::users;
use crate::domain::user::user::User;
use crate::domain::user::user_repository_abstract::UserRepositoryAbstract;
use crate::adapters::spi::db::{db_connection::DbConnection};


pub struct UserRepository {
    pub db_connection: DbConnection,
}

impl UserRepository {
    pub fn new(db_connection : DbConnection) -> Self{
        Self{
            db_connection : db_connection,
        }
    }
}


impl UserRepositoryAbstract for UserRepository {
    fn save(&self, user : User) -> Result<User, Box<dyn Error>>{
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");

        let result = diesel::insert_into(users::table)
            .values(&user)
            .get_result::<User>(&mut conn);

        match result {
            Ok(user) => {

                println!("user has been added");

                Ok(user)
            },
            Err(e) => Err(Box::new(e))
        }


    }
}