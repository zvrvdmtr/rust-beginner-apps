use std::sync::PoisonError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("item not found")]
    NotFound(String),
    #[error("failed to acquire lock {0}")]
    LockError(String),
}

impl<T> From<PoisonError<T>> for EngineError {
    fn from(value: PoisonError<T>) -> Self {
        EngineError::LockError(value.to_string())
    }
}
