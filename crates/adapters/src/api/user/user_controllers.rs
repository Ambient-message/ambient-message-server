use actix_web::{HttpResponse, web};
use actix_web::get;
use uuid::Uuid;

use application::usecases::user::create::CreateUserUseCase;
use application::usecases::user::interfaces::AbstractUseCase;
use domain::user::User;

use crate::api::shared::app_state::AppState;
use crate::api::shared::error_presenter::ErrorReponse;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user);
}

#[get("/")]
async fn create_user(data: web::Data<AppState>) -> Result<HttpResponse, ErrorReponse>{
    let user = User::new(Uuid::new_v4(), "dd", "dd");
    let create_user_usecase = CreateUserUseCase::new(user, &data.user_repository);
    let user = create_user_usecase.execute().await;

    user
        .map_err(ErrorReponse::map_io_error)
        .map(|_| HttpResponse::Ok().finish())
}