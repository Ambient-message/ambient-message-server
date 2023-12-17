use application::gateway::repositories::user_repository_abstract::{Record, Repo, SaveError};
use application::mappers::db_mapper::DbMapper;
use diesel::{RunQueryDsl, SelectableHelper};
use domain::entities::user::User;

use crate::db::db_connection::DbConnection;
use crate::db::models::UserModel;

use super::db_mappers::UserDbMapper;

impl Repo for DbConnection {
    fn save(&self, record: Record) -> Result<User, SaveError> {
        use super::schema::users;

        let Record { user: new_user } = record;
        let model = UserDbMapper::to_model(new_user);

        let res = diesel::insert_into(users::table)
            .values(&model)
            .returning(UserModel::as_returning())
            .get_result(&mut DbConnection::establish_connection())
            .expect("Error saving new user");

        Ok(UserDbMapper::to_entity(res))
    }
}
