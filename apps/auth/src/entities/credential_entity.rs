use serde::Serialize;
use sqlx::prelude::FromRow;

use chrono::{DateTime, Utc};

#[derive(FromRow, Serialize)]
pub struct Credential {
    pub id: String,
    pub user_id: String,

    pub password_hash: String,
    pub algorithm: String,

    pub updated_at: DateTime<Utc>,
}

pub struct CreateCredential {
    pub user_id: String,

    pub password_hash: String,
    pub algorithm: String,
}

pub struct UpdateCredential {
    pub password_hash: String,
    pub algorithm: String,
}
