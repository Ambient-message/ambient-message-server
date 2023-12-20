use application::mappers::api_mapper::ApiMapper;
use domain::user::User;
use crate::api::user::user_payloads::UserPayload;
use crate::api::user::user_presenters::UserPresenter;

pub struct UserMapper {}

impl ApiMapper<User, UserPresenter, UserPayload> for UserMapper {
    fn to_api(entity: User) -> UserPresenter {
        UserPresenter {
            username: entity.username,
            password: entity.password,
        }
    }

    fn to_entity(payload: UserPayload) -> User {
        User {
            id: payload.id,
            username: payload.username,
            password: payload.password,
        }
    }
}