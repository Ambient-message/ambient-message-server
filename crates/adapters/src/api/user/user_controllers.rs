use actix_web::get;
use actix_web::{HttpResponse, web};
use application::mappers::api_mapper::ApiMapper;
use application::usecases::user::create::CreateUser;
use application::usecases::user::interfaces::AbstractUseCase;
use diesel::RunQueryDsl;
use domain::user::User;
use uuid::Uuid;

use crate::api::shared::app_state::AppState;
use crate::api::shared::error_presenter::ErrorReponse;
use crate::api::user::user_mappers::UserMapper;
use crate::api::user::user_presenters::UserPresenter;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user);
}

#[get("/")]
async fn create_user(data: web::Data<AppState>) -> Result<HttpResponse, ErrorReponse>{
    let user = User::new(Uuid::new_v4(), "dd", "dd");
    let create_user = CreateUser::new(user, &data.user_repository);
    let user = create_user.execute().await;

    user
        .map_err(ErrorReponse::map_io_error)
        .map(|user_res| HttpResponse::Ok().json(UserMapper::to_api(user_res)))
}