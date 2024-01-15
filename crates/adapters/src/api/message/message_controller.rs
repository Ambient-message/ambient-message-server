use actix_web::{HttpResponse, post, web};
use uuid::Uuid;

use application::usecases::chat::create::CreateChatUseCase;
use application::usecases::chat::find::FindChatUseCase;
use application::usecases::interfaces::AbstractUseCase;
use domain::message_entity::MessageEntity;
use log::info;
use application::usecases::message::get_messages::GetMessagesUseCase;
use application::usecases::message::send::SendMessageUseCase;
use crate::api::message::message_payloads::MessagePayload;

use crate::api::shared::app_state::AppState;
use crate::api::shared::error_presenter::ErrorReponse;
use crate::api::user::user_controllers::get_all_users;
use crate::handlers::user_authentication::AuthenticatedUser;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(send_message).service(get_messages);
}

#[post("/message/send")]
async fn send_message(
    user: AuthenticatedUser,
    data: web::Data<AppState>,
    info: web::Json<MessagePayload>,
) -> Result<HttpResponse, ErrorReponse> {

    let message = MessageEntity{
        id: Uuid::new_v4(),
        sender_id: user.0,
        chat_id: info.chat_id,
        text: info.text.clone(),
    };

    let send_message_usecase = SendMessageUseCase::new(message, &data.message_repository);
    let res = send_message_usecase.execute().await;

    res.map_err(ErrorReponse::map_io_error)
        .map(|_| HttpResponse::Ok().finish())
}

#[post("/message/get")]
async fn get_messages(
    user: AuthenticatedUser,
    data: web::Data<AppState>,
    info: web::Json<Uuid>,
) -> Result<HttpResponse, ErrorReponse> {
    let get_messages_usecase = GetMessagesUseCase::new(info.0, &data.message_repository);
    let result = get_messages_usecase.execute().await;

    result.map_err(ErrorReponse::map_io_error)
        .map(|mes| {HttpResponse::Ok().json(mes) })
}