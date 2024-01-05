use std::fmt::Debug;
use std::sync::Arc;

use actix_web::{HttpResponse, ResponseError};
use actix_web::http::StatusCode;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde::de::Error;
use thiserror::Error;

use domain::api_error::ApiError;

#[derive(Serialize, Deserialize, Debug)]
struct ErrorPresenter {
    code: u16,
    message: String,
    error: String,
}

#[derive(Error, Debug, Display)]
#[display(fmt = "{:?}", error)]
pub struct ErrorReponse {
    status_code: Arc<actix_web::http::StatusCode>,
    error: String,
}

impl ResponseError for ErrorReponse {
    fn status_code(&self) -> StatusCode {
        *self.status_code.clone()
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorPresenter {
            code: status_code.as_u16(),
            message: status_code.to_string(),
            error: self.error.clone(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

impl ErrorReponse {
    pub fn map_io_error(e: ApiError) -> ErrorReponse {
        match e.get_error_code() {
            400 => ErrorReponse {
                status_code: StatusCode::BAD_REQUEST.into(),
                error: e.get_error_message(),
            },
            401 => ErrorReponse {
                status_code: StatusCode::UNAUTHORIZED.into(),
                error: e.get_error_message(),
            },
            403 => ErrorReponse {
                status_code: StatusCode::FORBIDDEN.into(),
                error: e.get_error_message(),
            },
            _ => ErrorReponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR.into(),
                error: String::from("Error: an unknown error occured"),
            },
        }
    }

    pub fn new(status_code: Arc<StatusCode>, error: String) -> Self {
        Self {
            status_code,
            error,
        }
    }
}