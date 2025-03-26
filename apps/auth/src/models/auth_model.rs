use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::entities::auth_refresh_token_entity::Session;

#[derive(Deserialize, Validate)]
pub struct RegisterWithCredentials {
    #[validate(
        email(message = "Email is invalid"),
        length(min = 1, message = "Email is required")
    )]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterWithCredentialsResponse {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub message: String,
}

#[derive(Deserialize, Validate)]
pub struct LoginWithCredentials {
    #[validate(
        email(message = "Email is invalid"),
        length(min = 1, message = "Email is required")
    )]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginWithCredentialsResponse {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub message: String,
}

#[derive(Deserialize, Validate)]
pub struct CheckEmailAvailabilityRequest {
    #[validate(
        email(message = "Email is invalid"),
        length(min = 1, message = "Email is required")
    )]
    pub email: String,
}

#[derive(Serialize)]
pub struct CheckEmailAvailabilityResponse {
    #[serde(rename = "isAvailable")]
    pub is_available: bool,

    pub email: String,
}

#[derive(Deserialize, Validate)]
pub struct ValidateOTPCodeRequest {
    #[validate(length(min = 6, max = 6, message = "OTP code must be 6 characters long"))]
    pub code: String,

    #[serde(rename = "userId")]
    #[validate(length(min = 1, message = "User ID is required"))]
    pub user_id: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    #[serde(rename = "accessToken")]
    access_token: String,

    #[serde(rename = "expiresAt")]
    expires_at: DateTime<Utc>,
}

impl From<Session> for TokenResponse {
    fn from(session: Session) -> Self {
        Self {
            access_token: session.access_token,
            expires_at: session.access_token_exp,
        }
    }
}

#[derive(Deserialize, Validate)]
pub struct SendConfirmEmailRequest {
    #[validate(length(min = 1, message = "User ID is required"))]
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Serialize)]
pub struct SendConfirmEmailResponse {
    pub message: String,
}

#[derive(Deserialize, Validate)]
pub struct ConfirmEmailRequest {
    #[validate(length(min = 1, message = "User ID is required"))]
    #[serde(rename = "userId")]
    pub user_id: String,

    #[validate(length(
        min = 1,
        message = "Confirmation code must be at least 1 character long"
    ))]
    pub token: String,
}
