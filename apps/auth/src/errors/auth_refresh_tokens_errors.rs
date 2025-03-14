use thiserror::Error;

use super::repository_errors::RepositoryError;

#[derive(Error, Debug)]
pub enum AuthRefreshTokensError {
    #[error("Ërror in repository: {0}")]
    Database(#[from] RepositoryError),

    #[error("JWT generation error: {0}")]
    JwtGenerationError(#[from] jsonwebtoken::errors::Error),

    #[error("Error hashing token")]
    HashRefreshToken,

    #[error("Error to refresh token")]
    InvalidRefreshToken,

    #[error("Error invalid user ID")]
    InvalidUserIdFormat,
}
