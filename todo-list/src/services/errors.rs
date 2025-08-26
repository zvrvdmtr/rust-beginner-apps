use std::{
    error::Error,
    fmt::{self},
};

use crate::repository::errors::RepositoryError;

#[derive(Debug)]
pub enum ApplicationError {
    NotFound { id: u32 },
    InternalError { message: String },
}

impl From<RepositoryError> for ApplicationError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::NotFound { id } => ApplicationError::NotFound { id: id },
            _ => ApplicationError::InternalError {
                message: "Internal Error".to_string(),
            },
        }
    }
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApplicationError::NotFound { id } => write!(f, "Record with id {id} not found"),
            ApplicationError::InternalError { message } => write!(f, "Internal error: {message}"),
        }
    }
}

impl Error for ApplicationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
