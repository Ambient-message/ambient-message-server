use actix_web::{HttpResponse, post, web};
use uuid::Uuid;

use application::usecases::chat::create::CreateChatUseCase;
use application::usecases::interfaces::AbstractUseCase;
use application::usecases::user_chat::create::CreateUserChatUseCase;
use domain::user_chat_entity::UserChatEntity;

use crate::api::shared::app_state::AppState;
use crate::api::shared::error_presenter::ErrorReponse;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_chat);
}

#[post("/chat/create")]
//todo receive user id by token
async fn create_chat(data: web::Data<AppState>, info: web::Json<Uuid>) -> Result<HttpResponse, ErrorReponse> {
    let create_chat_usecase = CreateChatUseCase::new(&data.chat_repository);

    let chat = create_chat_usecase.execute().await;

    match chat {
        Ok(chat) => {
            let user_chat = UserChatEntity::new(info.0, chat.id);

            let create_user_chat_usecase =
                CreateUserChatUseCase::new(user_chat, &data.user_chat_repository);

            let result = create_user_chat_usecase.execute().await;

            result
                .map_err(ErrorReponse::map_io_error)
                .map(|_| HttpResponse::Ok().finish())
        }
        Err(api_error) => {
            Err(ErrorReponse::map_io_error(api_error))
        }
    }
}