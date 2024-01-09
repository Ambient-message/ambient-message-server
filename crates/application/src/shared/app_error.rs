use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Empty password")]
    EmptyPassword,
    #[error("Wrong password")]
    WrongPassword,
    #[error("User not found")]
    UserNotFound,
    #[error("User not authorized")]
    NotAuthorized,
}
