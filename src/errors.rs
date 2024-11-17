use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("user email already been used: {0}")]
    EmailAlreadyExists(String),

    #[error("db error: {0}")]
    DBError(#[from] sqlx::Error),

    #[error("todo not exists: {0}")]
    TodoNotExists(String),

    #[error("user not exists: {0}")]
    UserNotFound(String),
}
