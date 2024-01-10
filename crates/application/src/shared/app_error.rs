use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Empty password")]
    EmptyPassword,
    #[error("Wrong password")]
    WrongPassword,
    #[error("User not exist")]
    UserNotExist,
    #[error("User not authorized")]
    NotAuthorized,
    #[error("User With This Username Already Exist")]
    UserWithThisUsernameAlreadyExist,
}
