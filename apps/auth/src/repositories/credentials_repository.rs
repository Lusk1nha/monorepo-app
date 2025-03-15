use crate::{
    database::DatabaseApp,
    entities::credential_entity::{CreateCredential, Credential, UpdateCredential},
    errors::repository_errors::RepositoryError,
};

#[derive(Clone)]
pub struct CredentialsRepository {
    database: DatabaseApp,
}

impl CredentialsRepository {
    const FIELDS: &'static str = "id, user_id, password_hash, algorithm, updated_at";
    const TABLE: &'static str = "credentials";

    pub fn new(database: DatabaseApp) -> Self {
        Self { database }
    }

    pub async fn find_by_id(&self, user_id: &str) -> Result<Option<Credential>, RepositoryError> {
        sqlx::query_as::<_, Credential>(&format!(
            "SELECT {} FROM {} WHERE user_id = $1",
            Self::FIELDS,
            Self::TABLE
        ))
        .bind(&user_id)
        .fetch_optional(&self.database.pool)
        .await
        .map_err(Into::into)
    }

    pub async fn create_credential(
        &self,
        id: &str,
        create_credential: &CreateCredential,
    ) -> Result<Credential, RepositoryError> {
        let mut tx = self.database.pool.begin().await?;

        let credential = sqlx::query_as::<_, Credential>(&format!(
            "INSERT INTO {} (id, user_id, password_hash, algorithm)
            VALUES ($1, $2, $3, $4) 
            RETURNING {}",
            Self::TABLE,
            Self::FIELDS
        ))
        .bind(&id)
        .bind(&create_credential.user_id)
        .bind(&create_credential.password_hash)
        .bind(&create_credential.algorithm)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                RepositoryError::UniqueViolation("id".into())
            }
            _ => RepositoryError::from(e),
        })?;

        tx.commit().await?;

        Ok(credential)
    }

    pub async fn update_credential(
        &self,
        user_id: &str,
        payload: &UpdateCredential,
    ) -> Result<Credential, RepositoryError> {
        let mut tx = self.database.pool.begin().await?;

        let credential = sqlx::query_as::<_, Credential>(&format!(
            "UPDATE {} SET password_hash = $2, algorithm = $3
            WHERE user_id = $1
            RETURNING {}",
            Self::TABLE,
            Self::FIELDS
        ))
        .bind(&user_id)
        .bind(&payload.password_hash)
        .bind(&payload.algorithm)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                RepositoryError::UniqueViolation("id".into())
            }
            _ => RepositoryError::from(e),
        })?;

        tx.commit().await?;

        Ok(credential)
    }
}
