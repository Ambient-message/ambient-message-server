use crate::adapters::spi::db::schema::users;
use crate::domain::user::user::User;
use crate::domain::user::user_repository::UserRepository;
use crate::adapters::spi::db::{db_connection::DbConnection};

impl UserRepository for DbConnection{

    fn by_id(&self,id: &str) -> Result<User,String> {
        todo!()
    }

    fn save(&self,client:User) {
        todo!()
    }

    fn next_identity(&self) -> String {
        todo!()
    }

    fn all(&self) -> Vec<User> {
        let conn = self.get_pool().get().expect("couldn't get db connection from pool");

//        let results = users.load::<User>(&conn);
//
//        match results {
//            Ok(models) => Ok(models.into_iter().map(DogFactDbMapper::to_entity).collect::<Vec<User>>()),
//            Err(e) => Err(Box::new(e)),
//        }

    }
}