use application::mappers::db_mapper::DbMapper;
use domain::user_entity::UserEntity;

use crate::spi::models::UserModel;

pub struct UserDbMapper {}

impl DbMapper<UserEntity, UserModel> for UserDbMapper {
    fn to_db(entity: UserEntity) -> UserModel {
        UserModel {
            id: entity.id,
            username: entity.username,
            password: entity.password,
        }
    }

    fn to_entity(model: UserModel) -> UserEntity {
        UserEntity {
            id: model.id,
            username: model.username,
            password: model.password,
        }
    }
}