use di::injectable;

use uuid::Uuid;
use std::error::Error;

use crate::adapters::spi::db::db_connection::*;

use crate::domain::user::user::User;
use crate::domain::user::user_repository_abstract::UserRepositoryAbstract;


#[injectable(UserRepositoryAbstract)]
pub struct UserRepository {
    pub db_context: Box<DbOptions>,
}

impl UserRepository {
   
}

impl UserRepositoryAbstract for UserRepository {
    fn save(&self, _user : User) -> Result<User, Box<dyn Error>>{

        Ok(User::new(Uuid::new_v4(), "s", "sad"))
//        let mut conn = self.db_context.get_pool().get().expect("couldn't get db connection from pool");
//
//        let result = diesel::insert_into(users::table)
//            .values(&user)
//            .get_result::<User>(&mut conn);
//
//        match result {
//            Ok(user) => {
//
//                println!("user has been added");
//
//                Ok(user)
//            },
//            Err(e) => Err(Box::new(e))
//        }
    }
}