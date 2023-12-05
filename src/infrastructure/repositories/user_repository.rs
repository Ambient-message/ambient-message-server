use crate::domain::user::{user_repository_abstract::UserRepositoryAbstract, user::User};


pub struct UserRepository {}

impl UserRepositoryAbstract for UserRepository {
    fn by_id(&self, id: i32) -> Result<User, String> {
        Ok(User::new(id, "ddd", "ddd"))
    }

    fn save(&self, client: User) {
        todo!()
    }

    fn next_identity(&self) -> String {
        todo!()
    }

    fn all(&self) -> Vec<User> {
        //let conn = self.get_pool().get().expect("couldn't get db connection from pool");
        todo!()
//        let results = users.load::<User>(&conn);
//
//        match results {
//            Ok(models) => Ok(models.into_iter().map(DogFactDbMapper::to_entity).collect::<Vec<User>>()),
//            Err(e) => Err(Box::new(e)),
//        }
    }
}