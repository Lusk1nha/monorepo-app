use serde::Serialize;
use sqlx::prelude::FromRow;

use chrono::{DateTime, Utc};

#[derive(Serialize, FromRow)]
pub struct EmailVerification {
    pub id: String,
    pub user_id: String,

    pub token: String,

    pub used_at: Option<DateTime<Utc>>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

pub struct CreateEmailVerification {
    pub user_id: String,
    pub token: String,
    pub expires_at: DateTime<Utc>,
}
