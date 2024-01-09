use std::fmt::Debug;

use actix_web::{HttpResponse, ResponseError};
use actix_web::http::StatusCode;
use anyhow::Error;
use derive_more::Display;
use serde::{Deserialize, Serialize};
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
    status_code: StatusCode,
    message: String,
    error: Error,
}

impl ResponseError for ErrorReponse {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code;
        let error_response = ErrorPresenter {
            code: status_code.as_u16(),
            message: self.message.clone(),
            error: self.error.to_string(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

impl ErrorReponse {
    pub fn map_io_error(err: ApiError) -> Self {
        Self {
            status_code: err.code,
            message: err.message,
            error: err.error,
        }
    }

    pub fn new(status_code: StatusCode, message: impl Into<String>, error: Error) -> Self {
        Self {
            status_code,
            message: message.into(),
            error,
        }
    }
}
