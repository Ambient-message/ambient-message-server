use actix_web::{HttpResponse, post, web};

use application::usecases::chat::create::CreateChatUseCase;
use application::usecases::interfaces::AbstractUseCase;

use crate::api::shared::app_state::AppState;
use crate::api::shared::error_presenter::ErrorReponse;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_chat);
}

#[post("/chat/create")]
async fn create_chat(data: web::Data<AppState>) -> Result<HttpResponse, ErrorReponse> {
    let create_chat_usecase = CreateChatUseCase::new(&data.chat_repository);
    let user = create_chat_usecase.execute().await;

    user
        .map_err(ErrorReponse::map_io_error)
        .map(|_| HttpResponse::Ok().finish())
}