use thiserror::Error;

use crate::utils::hmac_token;

use super::repository_errors::RepositoryError;

#[derive(Error, Debug)]
pub enum EmailVerificationsError {
    #[error("Error in Email verification repository: {0}")]
    Database(#[from] RepositoryError),

    #[error("Error in token generation: {0}")]
    TokenGeneration(#[from] hmac_token::HmacTokenError),

    #[error("Token is invalid or expired")]
    InvalidToken,

    #[error("Token is expired")]
    ExpiredToken,
}
