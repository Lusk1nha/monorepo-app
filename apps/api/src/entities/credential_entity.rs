use serde::Serialize;
use sqlx::prelude::FromRow;

use chrono::{DateTime, Utc};

#[derive(FromRow, Serialize)]
pub struct Credential {
    pub id: String,
    pub user_id: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct CreateCredential {
    pub id: String,
    pub user_id: String,
    pub email: String,
    pub password_hash: String,
}
