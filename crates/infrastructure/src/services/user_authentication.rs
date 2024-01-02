use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::error::{ErrorNotAcceptable, HttpError};
use actix_web::web::Data;
use actix_web::web::JsonBody::Error;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Error};
use futures::future::{BoxFuture, ready};
use uuid::Uuid;

use adapters::api::shared::app_state::AppState;
use adapters::api::shared::error_presenter::ErrorReponse;
use application::repositories::user_repository_abstract::UserRepositoryAbstract;
use domain::error::ApiError;

use crate::services::crypto::CryptoService;

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
                    let user_id: Uuid = crypto_service
                        .verify_jwt(bearer.token().to_string())
                        .await
                        .map(|data| data.claims.sub)
                        .map_err(|err| {
                            ErrorReponse::map_io_error(err)
                        })?;

                    repository.user_repository.find(user_id).map_err(|e| {
                        ErrorReponse::map_io_error(ApiError{
                            code: 400,
                            message: format!("Can't find {} user", user_id),
                            error: e,
                        })
                    })?;

                    Ok(AuthenticatedUser(user_id))
                };
                Box::pin(future)
            }
            _ => {
                let error = ready(Err(ErrorReponse::map_io_error(ApiError{
                    code: 400,
                    message: format!("NOT_AUTHORIZED"),
                    error: ,
                }).into()));
                Box::pin(error)
            }
        }
        }
}