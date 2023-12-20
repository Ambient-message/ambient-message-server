use application::mappers::db_mapper::DbMapper;
use domain::user::User;
use crate::spi::models::UserModel;

pub struct UserDbMapper {}

impl DbMapper<User, UserModel> for UserDbMapper {
    fn to_db(entity: User) -> UserModel {
        UserModel {
            id: entity.id,
            username: entity.username,
            password: entity.password,
        }
    }

    fn to_entity(model: UserModel) -> User {
        User {
            id: model.id,
            username: model.username,
            password: model.password,
        }
    }
}