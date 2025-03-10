use thiserror::Error;

#[derive(Error, Debug)]
pub enum CredentialsError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Password error: {0}")]
    Password(#[from] bcrypt::BcryptError),
}