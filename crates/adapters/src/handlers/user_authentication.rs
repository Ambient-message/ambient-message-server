use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use futures::future::{BoxFuture, ready};
use uuid::Uuid;

use application::repositories::user_repository_abstract::UserRepositoryAbstract;
use application::services::crypto_service_abstract::CryptoServiceAbstract;
use application::shared::app_error::AppError;

use crate::api::shared::app_state::AppState;
use crate::api::shared::error_presenter::ErrorReponse;
use crate::services::crypto::CryptoService;

//todo not service
pub struct AuthenticatedUser(pub Uuid);

impl FromRequest for AuthenticatedUser {
    type Error = ErrorReponse;
    type Future = BoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let bearer_result = BearerAuth::from_request(req, payload).into_inner();
        let repository_result = Data::<AppState>::from_request(req, payload).into_inner();
        let crypto_service_result = Data::<CryptoService>::from_request(req, payload).into_inner();

        match (bearer_result, repository_result, crypto_service_result) {
            (Ok(bearer), Ok(repository), Ok(crypto_service)) => {
                let future = async move {
                    let user_id = crypto_service
                        .verify_jwt(bearer.token().to_string())
                        .await
                        .map(|data| data.claims.sub)
                        .map_err(|err| ErrorReponse::map_io_error(err))?;

                    repository
                        .user_repository
                        .find_by_id(user_id)
                        .await.map_err(|err| ErrorReponse::map_io_error(err))?;

                    Ok(AuthenticatedUser { 0: user_id })
                };

                Box::pin(future)
            }
            _ => {
                let error = ready(Err(ErrorReponse::new(
                    StatusCode::UNAUTHORIZED,
                    "NOT AUTHORIZED",
                    AppError::NotAuthorized,
                )
                    .into()));
                Box::pin(error)
            }
        }
    }
}
