use crate::{
    database::DatabaseApp,
    entities::otp_code_entity::{CreateOTPCode, OTPCode},
    errors::repository_errors::RepositoryError,
};

#[derive(Clone)]
pub struct OTPCodesRepository {
    database: DatabaseApp,
}

impl OTPCodesRepository {
    const FIELDS: &'static str = "id, user_id, code, expires_at, created_at, used_at, is_used";
    const TABLE: &'static str = "otp_codes";

    pub fn new(database: DatabaseApp) -> Self {
        Self { database }
    }

    pub async fn find_active_user_otp(
        &self,
        user_id: &str,
    ) -> Result<Option<OTPCode>, RepositoryError> {
        sqlx::query_as::<_, OTPCode>(&format!(
            "SELECT {} FROM {} WHERE user_id = $1 AND is_used = false AND expires_at > now()",
            Self::FIELDS,
            Self::TABLE
        ))
        .bind(&user_id)
        .fetch_optional(&self.database.pool)
        .await
        .map_err(Into::into)
    }

    pub async fn create_otp_code(
        &self,
        create_otp_code: &CreateOTPCode,
    ) -> Result<OTPCode, RepositoryError> {
        let mut tx = self.database.pool.begin().await?;

        let otp_code = sqlx::query_as::<_, OTPCode>(&format!(
            "INSERT INTO {} (user_id, code, expires_at)
            VALUES ($1, $2, $3) 
            RETURNING {}",
            Self::TABLE,
            Self::FIELDS
        ))
        .bind(&create_otp_code.user_id)
        .bind(&create_otp_code.code)
        .bind(&create_otp_code.expires_at)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                RepositoryError::UniqueViolation("id".into())
            }
            _ => RepositoryError::from(e),
        })?;

        tx.commit().await?;

        Ok(otp_code)
    }

    pub async fn use_otp_code(&self, id: &i32, user_id: &str) -> Result<OTPCode, RepositoryError> {
        let mut tx = self.database.pool.begin().await?;

        let otp_code = sqlx::query_as::<_, OTPCode>(&format!(
            "UPDATE {} SET is_used = true, used_at = now()
            WHERE id = $1 AND user_id = $2
            RETURNING {}",
            Self::TABLE,
            Self::FIELDS
        ))
        .bind(&id)
        .bind(&user_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                RepositoryError::UniqueViolation("id".into())
            }
            _ => RepositoryError::from(e),
        })?;

        tx.commit().await?;

        Ok(otp_code)
    }
}
