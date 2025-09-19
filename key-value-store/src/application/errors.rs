use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

use crate::domain::errors::EngineError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("item not found")]
    NotFound(String),
    #[error("internal error")]
    InternalError(String)
}

impl From<EngineError> for AppError {
    fn from(value: EngineError) -> Self {
        match value {
            EngineError::NotFound(key) => AppError::NotFound(key),
            EngineError::LockError(value) => AppError::InternalError(value),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::NotFound(id) => (StatusCode::NOT_FOUND, format!("id {id} not found").to_string()).into_response(),
            AppError::InternalError(value) => (StatusCode::INTERNAL_SERVER_ERROR, value).into_response()
        }
    }
}
