use chrono::{DateTime, Utc};

use crate::{
    database::DatabaseApp,
    entities::auth_refresh_token_entity::{
        AuthRefreshToken, CreateAuthRefreshToken, UpdateAuthRefreshToken,
    },
    errors::repository_errors::RepositoryError,
};

#[derive(Clone)]
pub struct AuthRefreshTokensRepository {
    database: DatabaseApp,
}

impl AuthRefreshTokensRepository {
    const FIELDS: &'static str = "id, user_id, token_hash, expires_at, created_at, revoked_at";
    const TABLE: &'static str = "auth_refresh_tokens";

    pub fn new(database: DatabaseApp) -> Self {
        Self { database }
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<AuthRefreshToken>, RepositoryError> {
        sqlx::query_as::<_, AuthRefreshToken>(&format!(
            "SELECT {} FROM {} WHERE id = $1 AND revoked_at IS NULL",
            Self::FIELDS,
            Self::TABLE
        ))
        .bind(id)
        .fetch_optional(&*self.database.pool.lock().await)
        .await
        .map_err(Into::into)
    }

    pub async fn find_by_hash(
        &self,
        hash: &str,
    ) -> Result<Option<AuthRefreshToken>, RepositoryError> {
        sqlx::query_as::<_, AuthRefreshToken>(&format!(
            "SELECT {} FROM {} WHERE token_hash = $1 AND revoked_at IS NULL",
            Self::FIELDS,
            Self::TABLE
        ))
        .bind(&hash)
        .fetch_optional(&*self.database.pool.lock().await)
        .await
        .map_err(Into::into)
    }

    pub async fn store_refresh_token(
        &self,
        id: &str,
        payload: &CreateAuthRefreshToken,
    ) -> Result<AuthRefreshToken, RepositoryError> {
        let mut tx = self.database.pool.lock().await.begin().await?;

        let token = sqlx::query_as::<_, AuthRefreshToken>(&format!(
            "INSERT INTO {} (id, user_id, token_hash, expires_at) 
            VALUES ($1, $2, $3, $4)
            RETURNING {}",
            Self::TABLE,
            Self::FIELDS
        ))
        .bind(id)
        .bind(&payload.user_id)
        .bind(&payload.token_hash)
        .bind(payload.expires_at)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                RepositoryError::UniqueViolation("refresh_token".into())
            }
            _ => RepositoryError::from(e),
        })?;

        tx.commit().await?;

        Ok(token)
    }

    pub async fn update_refresh_token(
        &self,
        id: &str,
        payload: &UpdateAuthRefreshToken,
    ) -> Result<AuthRefreshToken, RepositoryError> {
        let mut tx = self.database.pool.lock().await.begin().await?;

        let token = sqlx::query_as::<_, AuthRefreshToken>(&format!(
            "UPDATE {} SET token_hash = $2, expires_at = $3
            WHERE id = $1 RETURNING {}",
            Self::TABLE,
            Self::FIELDS
        ))
        .bind(id)
        .bind(&payload.token_hash)
        .bind(&payload.expires_at)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                RepositoryError::UniqueViolation("refresh_token".into())
            }
            _ => RepositoryError::from(e),
        })?;

        tx.commit().await?;

        Ok(token)
    }

    pub async fn revoke_refresh_token(
        &self,
        id: &str,
        revoked_at: DateTime<Utc>,
    ) -> Result<(), RepositoryError> {
        let mut tx = self.database.pool.lock().await.begin().await?;

        sqlx::query(&format!(
            "UPDATE {} SET revoked_at = $2
            WHERE id = $1",
            Self::TABLE,
        ))
        .bind(&id)
        .bind(&revoked_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                RepositoryError::UniqueViolation("id".into())
            }
            _ => RepositoryError::from(e),
        })?;

        tx.commit().await?;

        Ok(())
    }
}
