use actix_web::{HttpResponse, post, web};
use uuid::Uuid;

use application::usecases::chat::create::CreateChatUseCase;
use application::usecases::interfaces::AbstractUseCase;

use crate::api::shared::app_state::AppState;
use crate::api::shared::error_presenter::ErrorReponse;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_chat);
}

#[post("/chat/create")]
//todo receive user id by token
async fn create_chat(data: web::Data<AppState>, info: web::Json<Uuid>) -> Result<HttpResponse, ErrorReponse> {
    let create_chat_usecase =
        CreateChatUseCase::new(info.0, &data.chat_repository, &data.user_chat_repository);
    let user = create_chat_usecase.execute().await;

    user
        .map_err(ErrorReponse::map_io_error)
        .map(|_| HttpResponse::Ok().finish())
}