use serde::Serialize;
use sqlx::prelude::FromRow;

use chrono::{DateTime, Utc};

#[derive(FromRow, Serialize)]
pub struct User {
    pub id: String,
    pub email: String,

    pub name: Option<String>,
    pub image: Option<String>,

    pub last_login_at: Option<DateTime<Utc>>,
    pub is_active: bool,

    pub is_2fa_enabled: bool,
    pub is_email_verified: bool,
    pub otp_secret: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct CreateUser {
    pub email: String,
    pub name: Option<String>,
    pub image: Option<String>,
}

pub struct UpdateUser {
    pub name: Option<String>,
    pub image: Option<String>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub is_active: bool,

    pub is_2fa_enabled: bool,
    pub is_email_verified: bool,
    pub otp_secret: Option<String>,
}
