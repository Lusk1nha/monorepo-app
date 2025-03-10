use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthProvidersError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}
