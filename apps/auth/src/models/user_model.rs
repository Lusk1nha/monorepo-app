use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::entities::user_entity::User;

#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,

    pub name: Option<String>,
    pub image: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl From<&User> for UserResponse {
    fn from(user: &User) -> Self {
        Self {
            id: user.id.clone(),
            email: user.email.clone(),
            name: user.name.clone(),
            image: user.image.clone(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Deserialize, Validate)]
pub struct UpdateEmailRequest {
    #[validate(email, length(min = 1, message = "Email is required"))]
    pub email: String,
}

#[derive(Serialize)]
pub struct UpdateEmailResponse {
    pub message: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdatePasswordRequest {
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,

    #[serde(rename = "newPassword")]
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub new_password: String,
}

#[derive(Serialize)]
pub struct UpdatePasswordResponse {
    pub message: String,
}
