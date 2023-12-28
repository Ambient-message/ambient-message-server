use async_trait::async_trait;
use uuid::Uuid;

use domain::chat_entity::ChatEntity;
use domain::error::ApiError;
use domain::user_chat_entity::UserChatEntity;

use crate::repositories::chat_repository_abstract::ChatRepositoryAbstract;
use crate::repositories::userchat_repository_abstract::UserChatRepositoryAbstract;
use crate::usecases::interfaces::AbstractUseCase;

pub struct CreateChatUseCase<'r, CR, UCR>
    where
        CR: ChatRepositoryAbstract,
        UCR: UserChatRepositoryAbstract,
{
    user_id: Uuid,
    chat_repository: &'r CR,
    user_chat_repository: &'r UCR,
}

impl<'r, CR, UCR> CreateChatUseCase<'r, CR, UCR>
    where
        CR: ChatRepositoryAbstract,
        UCR: UserChatRepositoryAbstract,
{
    pub fn new(user_id: Uuid, chat_repository: &'r CR, user_chat_repository: &'r UCR) -> Self {
        Self { user_id, chat_repository, user_chat_repository }
    }
}

#[async_trait(? Send)]
impl<'r, CR, UCR> AbstractUseCase<()> for CreateChatUseCase<'r, CR, UCR>
    where
        CR: ChatRepositoryAbstract,
        UCR: UserChatRepositoryAbstract,
{
    async fn execute(&self) -> Result<(), ApiError> {
        let chat = ChatEntity::new();
        let result = self.chat_repository.save(chat);

        match result {
            Ok(chat) => {
                let user_chat = UserChatEntity::new(self.user_id, chat.id);

                let result = self.user_chat_repository.save(&user_chat);

                match result {
                    Ok(_) => Ok(()),
                    Err(e) => Err(ApiError {
                        code: 400,
                        message: String::from("Cannot create user chat"),
                        error: e,
                    }),
                }
            }
            Err(e) => Err(ApiError {
                code: 400,
                message: String::from("Cannot create chat"),
                error: e,
            }),
        }
    }
}