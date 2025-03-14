use serde::Serialize;
use sqlx::prelude::FromRow;

use chrono::{DateTime, Utc};

#[derive(FromRow, Serialize)]
pub struct AuthProvider {
    pub id: i32,
    pub user_id: String,

    pub provider: String,
    pub provider_user_id: String,

    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_expires_at: Option<DateTime<Utc>>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct CreateAuthProvider {
    pub user_id: String,
    pub provider: String,
    pub provider_user_id: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_expires_at: Option<DateTime<Utc>>,
}
