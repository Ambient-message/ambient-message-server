use actix_web::{get, HttpResponse, post, web};
use actix_web_httpauth::extractors::basic::BasicAuth;
use serde::{Deserialize, Serialize};

use application::mappers::api_mapper::ApiMapper;
use application::usecases::interfaces::AbstractUseCase;
use application::usecases::user::auth::AuthUserUseCase;
use application::usecases::user::create::CreateUserUseCase;
use application::usecases::user::find::FindUserByIDUseCase;
use application::usecases::user::get_all::GetAllUsersUseCase;

use crate::api::shared::app_state::AppState;
use crate::api::shared::error_presenter::ErrorReponse;
use crate::api::user::user_mappers::UserMapper;
use crate::api::user::user_payloads::UserPayload;
use crate::api::user::user_presenters::UserPresenter;
use crate::handlers::user_authentication::AuthenticatedUser;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user).service(user_profile).service(auth).service(get_all_users);
}

#[post("/reg")]
async fn create_user(
    basic: BasicAuth,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ErrorReponse> {
    println!("creating user {}", basic.user_id());

    let create_user_usecase = CreateUserUseCase::new(basic.clone(), &data.user_repository, &data.crypto_services);
    create_user_usecase.execute().await.map_err(ErrorReponse::map_io_error)?;

    let auth_user_usecase
        = AuthUserUseCase::new(basic, &data.user_repository, &data.crypto_services);

    let token = auth_user_usecase
        .execute()
        .await
        .map_err(ErrorReponse::map_io_error)?;

    let user_info = UserInfo {
        id: token.0.to_string(),
        token: token.1,
    };

    Ok(HttpResponse::Ok().json(user_info))
}

#[post("/user/profile")]
async fn user_profile(
    user: AuthenticatedUser,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ErrorReponse> {
    let find_user_usecase = FindUserByIDUseCase::new(user.0, &data.user_repository);
    let user = find_user_usecase.execute().await;

    user.map_err(ErrorReponse::map_io_error)
        .map(|user| HttpResponse::Ok().json(UserMapper::to_api(user)))
}

#[get("/user/get_all")]
async fn get_all_users(
    data: web::Data<AppState>,
) -> Result<HttpResponse, ErrorReponse> {
    let get_all_users_usecase = GetAllUsersUseCase::new(&data.user_repository);
    let list = get_all_users_usecase.execute().await;

    let list = list
        .map_err(ErrorReponse::map_io_error)?
        .iter()
        .map(|user| UserMapper::to_api(user.clone()))
        .collect::<Vec<UserPresenter>>();

    println!("Getting all users");
    Ok(HttpResponse::Ok().json(list))
}

#[post("/auth")]
async fn auth(
    basic: BasicAuth,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ErrorReponse> {
    let auth_user_usecase
        = AuthUserUseCase::new(basic, &data.user_repository, &data.crypto_services);
    let token = auth_user_usecase
        .execute()
        .await
        .map_err(ErrorReponse::map_io_error)?;

    let user_info = UserInfo {
        id: token.0.to_string(),
        token: token.1,
    };

    Ok(HttpResponse::Ok().json(user_info))
}

//todo use payload
#[derive(Serialize, Deserialize)]
struct UserInfo {
    id: String,
    token: String,
}