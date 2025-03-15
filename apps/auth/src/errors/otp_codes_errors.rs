use thiserror::Error;

use super::repository_errors::RepositoryError;

#[derive(Error, Debug)]
pub enum OTPCodesError {
    #[error("Error in OTP code repository: {0}")]
    Database(#[from] RepositoryError),
    
    #[error("Error to generate OTP code")]
    GenerateCode,
}
