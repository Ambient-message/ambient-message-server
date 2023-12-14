use super::models::UserModel;
use application::mappers::db_mapper::DbMapper;
use domain::entities::user::User;

pub struct UserDbMapper {}

impl DbMapper<User, UserModel> for UserDbMapper {
    
    fn to_model(entity: User) -> UserModel {
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