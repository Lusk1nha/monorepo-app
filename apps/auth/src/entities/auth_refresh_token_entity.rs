use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow, Serialize)]
pub struct AuthRefreshToken {
    pub id: String,
    pub user_id: String,

    pub token_hash: String,
    pub expires_at: DateTime<Utc>,

    pub created_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,

    pub iat: usize,
}

pub struct CreateAuthRefreshToken {
    pub user_id: String,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
}

pub struct UpdateAuthRefreshToken {
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
}

pub struct Session {
    pub access_token: String,
    pub refresh_token: String,

    pub access_token_exp: DateTime<Utc>,
    pub refresh_token_exp: DateTime<Utc>
}
