use async_trait::async_trait;

use domain::error::ApiError;
use domain::user_chat_entity::UserChatEntity;

use crate::repositories::userchat_repository_abstract::UserChatRepositoryAbstract;
use crate::usecases::interfaces::AbstractUseCase;

pub struct CreateUserChatUseCase<'r, R>
    where
        R: UserChatRepositoryAbstract,
{
    user_chat: UserChatEntity,
    repository: &'r R,
}

impl<'r, R> CreateUserChatUseCase<'r, R>
    where
        R: UserChatRepositoryAbstract,
{
    pub fn new(user_chat: UserChatEntity, repository: &'r R) -> Self {
        Self { user_chat, repository }
    }
}

#[async_trait(? Send)]
impl<'r, R> AbstractUseCase<()> for CreateUserChatUseCase<'r, R>
    where
        R: UserChatRepositoryAbstract
{
    async fn execute(&self) -> Result<(), ApiError> {
        let result = self.repository.save(&self.user_chat);

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(ApiError {
                code: 400,
                message: String::from("Cannot create user chat"),
                error: e,
            }),
        }
    }
}