use std::fmt;

use actix_web::http::StatusCode;
use anyhow::Error;

#[derive(Debug)]
pub struct ApiError {
    pub code: StatusCode,
    pub message: String,
    pub error: Error,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

impl ApiError {
    pub fn get_error_message(&self) -> String {
        String::from(&self.message)
    }

    pub fn get_error_code(&self) -> u16 {
        self.code.as_u16()
    }

    pub fn new(code: StatusCode, message: impl Into<String>, error: impl Into<Error>) -> Self {
        Self {
            code,
            message: message.into(),
            error: error.into(),
        }
    }
}
