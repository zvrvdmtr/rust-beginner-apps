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