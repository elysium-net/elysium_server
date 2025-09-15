use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};
use tonic::Status;

#[derive(Debug, Serialize, Deserialize)]
pub enum ElyError {
    UserNotFound,
    UserAlreadyExists,
    SessionNotFound,
    Unauthorized,
    InvalidToken,
    PostNotFound,
    EmailNotVerified,
    Database,
}

impl Display for ElyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ElyError::Database => write!(f, "Internal Database Error"),
            ElyError::UserNotFound => write!(f, "User not found"),
            ElyError::SessionNotFound => write!(f, "Session not found"),
            ElyError::UserAlreadyExists => write!(f, "User already exists"),
            ElyError::Unauthorized => write!(f, "Unauthorized"),
            ElyError::InvalidToken => write!(f, "Invalid token"),
            ElyError::PostNotFound => write!(f, "Post not found"),
            ElyError::EmailNotVerified => write!(f, "Email could not be verified"),
        }
    }
}

impl Error for ElyError {}

impl From<surrealdb::Error> for ElyError {
    fn from(e: surrealdb::Error) -> Self {
        tracing::error!("Database Error: {}", e);
        ElyError::Database
    }
}

impl From<jsonwebtoken::errors::Error> for ElyError {
    fn from(_: jsonwebtoken::errors::Error) -> Self {
        ElyError::InvalidToken
    }
}

impl From<ElyError> for Status {
    fn from(value: ElyError) -> Self {
        let msg = format!("{}.", value);

        match value {
            ElyError::UserNotFound => Status::not_found(msg),
            ElyError::UserAlreadyExists => Status::already_exists(msg),
            ElyError::SessionNotFound => Status::not_found(msg),
            ElyError::Unauthorized => Status::unauthenticated(msg),
            ElyError::InvalidToken => Status::invalid_argument(msg),
            ElyError::PostNotFound => Status::not_found(msg),
            ElyError::EmailNotVerified => Status::not_found(msg),
            ElyError::Database => Status::internal(msg),
        }
    }
}
