use actix_web::{HttpResponse, post, web};
use actix_web::http::StatusCode;
use actix_web_httpauth::extractors::basic::BasicAuth;

use application::mappers::api_mapper::ApiMapper;
use application::services::crypto_service_abstract::CryptoServiceAbstract;
use application::usecases::interfaces::AbstractUseCase;
use application::usecases::user::auth::AuthUserUseCase;
use application::usecases::user::create::CreateUserUseCase;
use application::usecases::user::find::FindUserUseCase;

use crate::api::shared::app_state::AppState;
use crate::api::shared::error_presenter::ErrorReponse;
use crate::api::user::user_mappers::UserMapper;
use crate::api::user::user_payloads::UserPayload;
use crate::handlers::user_authentication::AuthenticatedUser;
use crate::services::crypto::CryptoService;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user).service(user_profile).service(auth);
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
async fn user_profile(user: AuthenticatedUser, data: web::Data<AppState>) -> Result<HttpResponse, ErrorReponse> {
    let find_user_usecase = FindUserUseCase::new(user.0, &data.user_repository);
    let user = find_user_usecase.execute().await;

    user
        .map_err(ErrorReponse::map_io_error)
        .map(|user| HttpResponse::Ok().json(UserMapper::to_api(user)))
}

#[post("/auth/v2")]
async fn auth(basic: BasicAuth, data: web::Data<AppState>, hashing: web::Data<CryptoService>)
              -> Result<HttpResponse, ErrorReponse> {
    let auth_user_usecase = AuthUserUseCase::new(basic, &data.user_repository, hashing.get_ref());
    let token = auth_user_usecase.execute().await;

    match token {
        Ok(token) => {
            match token {
                Some(token) => Ok(HttpResponse::Ok().json(token)),
                None => Err(ErrorReponse::new(StatusCode::UNAUTHORIZED.into(), String::from("User not found"))),
            }
        }
        Err(e) => Err(ErrorReponse::map_io_error(e))
    }
}