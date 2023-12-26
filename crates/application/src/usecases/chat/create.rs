use async_trait::async_trait;

use domain::chat_entity::ChatEntity;
use domain::error::ApiError;

use crate::repositories::chat_repository_abstract::ChatRepositoryAbstract;
use crate::usecases::interfaces::AbstractUseCase;

pub struct CreateChatUseCase<'r, R>
    where
        R: ChatRepositoryAbstract,
{
    repository: &'r R,
}

impl<'r, R> CreateChatUseCase<'r, R>
    where
        R: ChatRepositoryAbstract,
{
    pub fn new(repository: &'r R) -> Self {
        Self { repository }
    }
}

#[async_trait(? Send)]
impl<'r, R> AbstractUseCase<()> for CreateChatUseCase<'r, R>
    where
        R: ChatRepositoryAbstract,
{
    async fn execute(&self) -> Result<(), ApiError> {
        let chat = ChatEntity::new();
        let result = self.repository.save(chat);

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(ApiError {
                code: 400,
                message: String::from("Cannot create chat"),
                error: e,
            }),
        }
    }
}