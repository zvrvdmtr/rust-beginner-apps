use crate::repository::errors::RepositoryError;

#[derive(Debug)]
pub enum ApplicationError {
    NotFound { id: u32 },
    InternalError { message: String },
}

impl From<RepositoryError> for ApplicationError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::NotFound{ id } => ApplicationError::NotFound {id: id},
            _ => ApplicationError::InternalError{ message: "Internal Error".to_string() }

        }
    }
}