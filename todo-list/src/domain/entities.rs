use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub body: String,
}

#[derive(Debug)]
pub enum RepositoryError {
    Io(std::io::Error),
    CsvParse(csv::Error),
    CsvIo(csv::Error),
    CsvOther(String),
    InvalidId { value: String },
    NotFound { id: u32 },
}

impl From<csv::Error> for RepositoryError {
    fn from(err: csv::Error) -> Self {
        match err.kind() {
            csv::ErrorKind::Io(e) => RepositoryError::CsvIo(err),
            csv::ErrorKind::UnequalLengths{ .. } => RepositoryError::CsvParse(err),
            _ => RepositoryError::CsvOther(err.to_string())

        }
    }
}

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

#[derive(Debug)]
pub enum Order {
    Asc,
    Desc,
}

impl FromStr for Order {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "asc" => Ok(Order::Asc),
            "desc" => Ok(Order::Desc),
            _ => Err("Invalid order value. Available values is: \"asc\", \"desc\"".to_string()),
        }
    }
}

#[derive(Debug)]
pub enum Action {
    Get,
    List,
    Add,
    Delete,
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "get" => Ok(Action::Get),
            "list" => Ok(Action::List),
            "add" => Ok(Action::Add),
            "delete" => Ok(Action::Delete),
            _ => Err(
                "Invalid action value. Please use one of: \"get\", \"list\", \"add\" or \"delete\""
                    .to_string(),
            ),
        }
    }
}
