use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Database connection error: {0}")]
    Connection(sqlx::Error),

    #[error("{0} not found")]
    NotFound(String),

    #[error("Unique constraint violation on {0}")]
    UniqueViolation(String),

    #[error("Invalid data format")]
    DataFormat(#[from] chrono::ParseError),

    #[error("Transaction error")]
    Transaction,
}

impl From<sqlx::Error> for RepositoryError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound("record".into()),
            _ => RepositoryError::Connection(e),
        }
    }
}
