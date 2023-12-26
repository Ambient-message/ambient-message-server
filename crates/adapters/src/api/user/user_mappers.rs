use application::mappers::api_mapper::ApiMapper;
use domain::user_entity::UserEntity;

use crate::api::user::user_payloads::UserPayload;
use crate::api::user::user_presenters::UserPresenter;

pub struct UserMapper {}

impl ApiMapper<UserEntity, UserPresenter, UserPayload> for UserMapper {
    fn to_api(entity: UserEntity) -> UserPresenter {
        UserPresenter {
            id: entity.id,
            username: entity.username,
        }
    }

    fn to_entity(payload: UserPayload) -> UserEntity {
        UserEntity::new(payload.username, payload.password)
    }
}