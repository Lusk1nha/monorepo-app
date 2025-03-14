use thiserror::Error;

use super::repository_errors::RepositoryError;

#[derive(Error, Debug)]
pub enum AuthProvidersError {
    #[error("Database error: {0}")]
    Database(#[from] RepositoryError),
}
