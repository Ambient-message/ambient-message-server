use uuid::Uuid;

use domain::api_error::ApiError;
use domain::chat_entity::ChatEntity;
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
    second_user_id: Uuid,
    chat_repository: &'r CR,
    user_chat_repository: &'r UCR,
}

impl<'r, CR, UCR> CreateChatUseCase<'r, CR, UCR>
    where
        CR: ChatRepositoryAbstract,
        UCR: UserChatRepositoryAbstract,
{
    pub fn new(user_id: Uuid, second_user_id: Uuid, chat_repository: &'r CR, user_chat_repository: &'r UCR) -> Self {
        Self {
            user_id,
            second_user_id,
            chat_repository,
            user_chat_repository,
        }
    }
}

impl<'r, CR, UCR> AbstractUseCase<()> for CreateChatUseCase<'r, CR, UCR>
    where
        CR: ChatRepositoryAbstract,
        UCR: UserChatRepositoryAbstract,
{
    async fn execute(&self) -> Result<(), ApiError> {
        let chat = ChatEntity::new();
        println!("create: {}  with user {} and {}", chat.id, self.user_id, self.second_user_id);
        let chat = self.chat_repository.save(chat)?;

        let user_chat = UserChatEntity::new(self.user_id, chat.id);
        self.user_chat_repository.save(&user_chat)?;

        let user_chat = UserChatEntity::new(self.second_user_id, chat.id);
        self.user_chat_repository.save(&user_chat)?;

        Ok(())
    }
}
