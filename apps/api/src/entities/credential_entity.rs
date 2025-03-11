use serde::Serialize;
use sqlx::prelude::FromRow;

use chrono::{DateTime, Utc};

#[derive(FromRow, Serialize)]
pub struct Credential {
    pub id: String,
    pub user_id: String,

    pub password_hash: String,
    pub alghorithm: String,

    pub updated_at: DateTime<Utc>,
}

pub struct CreateCredential {
    pub id: String,
    pub user_id: String,

    pub password_hash: String,
    pub alghorithm: String,
}
