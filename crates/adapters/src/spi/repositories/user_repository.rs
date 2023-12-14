use std::error::Error;

use application::gateway::repositories::user_repository_abstract::{
    Record, UserRepositoryAbstract,
};
use domain::entities::user::User;

use crate::spi::db::db_connection::DbConnection;

pub struct UserRepository {
    pub db_connection: DbConnection,
}

// impl UserRepositoryAbstract for UserRepository {
//     fn save(&self, record: Record) -> Result<User, Box<dyn Error>> {
//         let conn = self
//             .db_connection
//             .get_pool()
//             .get()
//             .expect("couldn't get db connection from pool");
//     }
// }
