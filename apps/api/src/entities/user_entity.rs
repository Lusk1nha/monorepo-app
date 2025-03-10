use serde::Serialize;
use sqlx::prelude::FromRow;

use chrono::{DateTime, Utc};

#[derive(FromRow, Serialize)]
pub struct User {
    pub id: String,
    pub name: Option<String>,
    pub image: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub last_login_at: Option<DateTime<Utc>>,
    is_active: bool,
}

pub struct CreateUser {
    pub id: String,
    pub name: Option<String>,
    pub image: Option<String>,
}

pub struct UpdateUser {
    pub id: String,
    pub name: Option<String>,
    pub image: Option<String>,
}

pub struct UserLogin {
    pub id: String,
    pub last_login_at: DateTime<Utc>,
}
