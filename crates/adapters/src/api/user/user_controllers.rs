use actix_web::{HttpResponse, post, web};
use uuid::Uuid;

use application::mappers::api_mapper::ApiMapper;
use application::usecases::interfaces::AbstractUseCase;
use application::usecases::user::create::CreateUserUseCase;
use application::usecases::user::find::FindUserUseCase;

use crate::api::shared::app_state::AppState;
use crate::api::shared::error_presenter::ErrorReponse;
use crate::api::user::user_mappers::UserMapper;
use crate::api::user::user_payloads::UserPayload;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user).service(user_profile);
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

#[post("/user/profile")]
async fn user_profile(data: web::Data<AppState>, info: web::Json<Uuid>) -> Result<HttpResponse, ErrorReponse> {

    let create_user_usecase = FindUserUseCase::new(info.0, &data.user_repository);
    let user = create_user_usecase.execute().await;

    user
        .map_err(ErrorReponse::map_io_error)
        .map(|user| HttpResponse::Ok().json(UserMapper::to_api(user)))
}