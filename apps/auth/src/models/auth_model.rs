use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::entities::{auth_refresh_token_entity::Session, user_entity::User};

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
    pub id: String,
    pub message: String,
}

impl From<User> for RegisterWithCredentialsResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            message: "User created successfully".to_string(),
        }
    }
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
    #[serde(rename = "accessToken")]
    access_token: String,

    #[serde(rename = "expiresAt")]
    expires_at: DateTime<Utc>,
}

impl From<Session> for LoginWithCredentialsResponse {
    fn from(session: Session) -> Self {
        Self {
            access_token: session.access_token,
            expires_at: session.access_token_exp,
        }
    }
}
