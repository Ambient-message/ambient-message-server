use di::injectable;
use diesel::RunQueryDsl;
use std::error::Error;
use std::rc::Rc;
use crate::adapters::spi::db::db_connection::*;
use crate::adapters::spi::db::schema::users;
use crate::domain::user::user::User;
use crate::domain::user::user_repository_abstract::UserRepositoryAbstract;


#[injectable(UserRepositoryAbstract)]
pub struct UserRepository {
    pub db_context: Rc<dyn DbContext>,
}

impl UserRepository {
    pub fn new(db_context : Rc<dyn DbContext>) -> Self{
        Self{
            db_context : db_context,
        }
    }
}

impl UserRepositoryAbstract for UserRepository {
    fn save(&self, user : User) -> Result<User, Box<dyn Error>>{

        let mut conn = self.db_context.get_pool().get().expect("couldn't get db connection from pool");

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