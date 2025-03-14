use thiserror::Error;

use super::repository_errors::RepositoryError;

#[derive(Error, Debug)]
pub enum CredentialsError {
    #[error("Database error: {0}")]
    Database(#[from] RepositoryError),

    #[error("Password error: {0}")]
    Password(#[from] bcrypt::BcryptError),
}