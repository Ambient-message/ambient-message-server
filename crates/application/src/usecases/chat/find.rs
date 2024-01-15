use uuid::Uuid;

use domain::api_error::ApiError;
use domain::chat_entity::ChatEntity;
use domain::user_chat_entity::UserChatEntity;

use crate::repositories::chat_repository_abstract::ChatRepositoryAbstract;
use crate::repositories::userchat_repository_abstract::UserChatRepositoryAbstract;
use crate::usecases::interfaces::AbstractUseCase;

pub struct FindChatUseCase<'r, UCR>
    where
        UCR: UserChatRepositoryAbstract,
{
    first_user_id: Uuid,
    second_user_id: Uuid,
    user_chat_repository: &'r UCR,
}

impl<'r, UCR> FindChatUseCase<'r, UCR>
    where
        UCR: UserChatRepositoryAbstract,
{
    pub fn new(first_user_id: Uuid, second_user_id: Uuid, user_chat_repository: &'r UCR) -> Self {
        Self {
            first_user_id,
            second_user_id,
            user_chat_repository,
        }
    }
}

impl<'r, UCR> AbstractUseCase<Option<Uuid>> for FindChatUseCase<'r, UCR>
    where
        UCR: UserChatRepositoryAbstract,
{
    async fn execute(&self) -> Result<Option<Uuid>, ApiError> {
        self.user_chat_repository.find(self.first_user_id, self.second_user_id)
    }
}
