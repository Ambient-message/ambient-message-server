use application::mappers::api_mapper::ApiMapper;
use application::repositories::user_repository_abstract::UserRepositoryAbstract;
use domain::api_error::ApiError;
use domain::user_entity::UserEntity;

use crate::api::user::user_payloads::UserPayload;
use crate::api::user::user_presenters::UserPresenter;

pub struct UserMapper<'r, R>
where R: UserRepositoryAbstract{
    pub user_repository: &'r  R,
}

impl<'r, R> ApiMapper<UserEntity, UserPresenter, UserPayload, ApiError> for UserMapper<'r, R>
where R: UserRepositoryAbstract{
    fn to_api(entity: UserEntity) -> UserPresenter {
        UserPresenter {
            id: entity.id,
            username: entity.username,
        }
    }

    async fn to_entity(&self, payload: UserPayload) -> Result<Option<UserEntity>, ApiError> {
        self.user_repository.find_by_username(payload.username).await
    }
}
