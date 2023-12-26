use actix_web::{HttpResponse, post, web};

use application::mappers::api_mapper::ApiMapper;
use application::usecases::user::create::CreateUserUseCase;
use application::usecases::user::interfaces::AbstractUseCase;

use crate::api::shared::app_state::AppState;
use crate::api::shared::error_presenter::ErrorReponse;
use crate::api::user::user_mappers::UserMapper;
use crate::api::user::user_payloads::UserPayload;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user);
}

#[post("/auth")]
async fn create_user(data: web::Data<AppState>, info: web::Json<UserPayload>) -> Result<HttpResponse, ErrorReponse> {
    let user = UserMapper::to_entity(info.0);

    println!("{}", user.username);

    let create_user_usecase = CreateUserUseCase::new(user, &data.user_repository);
    let user = create_user_usecase.execute().await;

    user
        .map_err(ErrorReponse::map_io_error)
        .map(|_| HttpResponse::Ok().finish())
}