use crate::{
    database::DatabaseApp,
    entities::email_verification_entity::{CreateEmailVerification, EmailVerification},
    errors::repository_errors::RepositoryError,
};

#[derive(Clone)]
pub struct EmailVerificationsRepository {
    database: DatabaseApp,
}

impl EmailVerificationsRepository {
    const FIELDS: &'static str = "id, user_id, token, expires_at, used_at, created_at";
    const TABLE: &'static str = "email_verification_tokens";

    pub fn new(database: DatabaseApp) -> Self {
        Self { database }
    }

    pub async fn find_by_user_id(
        &self,
        user_id: &str,
    ) -> Result<Option<EmailVerification>, RepositoryError> {
        sqlx::query_as::<_, EmailVerification>(&format!(
            "SELECT {} FROM {} WHERE user_id = $1 AND used_at IS NULL",
            Self::FIELDS,
            Self::TABLE
        ))
        .bind(&user_id)
        .fetch_optional(&*self.database.pool.lock().await)
        .await
        .map_err(Into::into)
    }

    pub async fn store_email_verification(
        &self,
        id: String,
        payload: &CreateEmailVerification,
    ) -> Result<EmailVerification, RepositoryError> {
        let mut tx = self.database.pool.lock().await.begin().await?;

        let email_verification = sqlx::query_as::<_, EmailVerification>(&format!(
            "INSERT INTO {} (id, user_id, token, expires_at)
            VALUES ($1, $2, $3, $4) 
            RETURNING {}",
            Self::TABLE,
            Self::FIELDS
        ))
        .bind(&id)
        .bind(&payload.user_id)
        .bind(&payload.token)
        .bind(&payload.expires_at)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                RepositoryError::UniqueViolation("id".into())
            }
            _ => RepositoryError::from(e),
        })?;

        tx.commit().await?;

        Ok(email_verification)
    }

    pub async fn mark_as_used(&self, user_id: &str, token: &str) -> Result<EmailVerification, RepositoryError> {
        let mut tx = self.database.pool.lock().await.begin().await?;

        let email_verification = sqlx::query_as::<_, EmailVerification>(&format!(
            "UPDATE {} SET used_at = now() WHERE user_id = $1 AND token = $2 AND used_at IS NULL RETURNING {}",
            Self::TABLE,
            Self::FIELDS
        ))
        .bind(user_id)
        .bind(token)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                RepositoryError::UniqueViolation("id".into())
            }
            _ => RepositoryError::from(e),
        })?;

        tx.commit().await?;

        Ok(email_verification)
    }
}
