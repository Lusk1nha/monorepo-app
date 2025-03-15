use thiserror::Error;

use super::otp_codes_errors::OTPCodesError;

#[derive(Error, Debug)]
pub enum UsersError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Error to generate OTP secret: {0}")]
    OTPSecret(#[from] OTPCodesError),
}
