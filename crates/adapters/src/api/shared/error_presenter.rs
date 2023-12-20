use std::fmt::{Debug, Display, Formatter};
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use domain::error::ApiError;
use thiserror::Error;
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ErrorPresenter {
    code: u16,
    message: String,
    error: String
}

#[derive(Error, Debug, Display)]
#[display(fmt = "{:?}", error)]
pub struct ErrorReponse {
    status_code: StatusCode,
    error: String,
}

impl ResponseError for ErrorReponse {
    fn status_code(&self) -> StatusCode {
        self.status_code
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
                status_code: StatusCode::BAD_REQUEST,
                error: e.get_error_message(),
            },
            401 => ErrorReponse {
                status_code: StatusCode::UNAUTHORIZED,
                error: e.get_error_message(),
            },
            403 => ErrorReponse {
                status_code: StatusCode::FORBIDDEN,
                error: e.get_error_message(),
            },
            _ => ErrorReponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                error: String::from("Error: an unknown error occured"),
            },
        }
    }
}