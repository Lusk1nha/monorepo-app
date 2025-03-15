use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Serialize, Clone, FromRow)]
pub struct OTPCode {
    pub id: i32,
    pub user_id: String,

    pub code: String,
    pub expires_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
    pub is_used: bool,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct CreateOTPCode {
    pub user_id: String,
    pub code: String,
    pub expires_at: DateTime<Utc>,
}

pub struct UpdateOTPCode {
    pub used_at: DateTime<Utc>,
    pub is_used: bool,
}
