use serde::Serialize;
use sqlx::prelude::FromRow;

use chrono::{DateTime, Utc};

#[derive(FromRow, Serialize)]
pub struct AuthProvider {
    pub id: String,
    pub provider_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct AuthProviderCreate {
    pub id: String,
    pub provider_type: String,
}
