use crate::{
    database::DatabaseApp,
    entities::auth_provider_entity::{AuthProvider, CreateAuthProvider},
    errors::repository_errors::RepositoryError,
};

#[derive(Clone)]
pub struct AuthProvidersRepository {
    database: DatabaseApp,
}

impl AuthProvidersRepository {
    const FIELDS: &'static str = "id, user_id, provider, provider_user_id, access_token, refresh_token, token_expires_at, created_at, updated_at";
    const TABLE: &'static str = "auth_providers";

    pub fn new(database: DatabaseApp) -> Self {
        Self { database }
    }

    pub async fn create_auth_provider(
        &self,
        create_auth_provider: &CreateAuthProvider,
    ) -> Result<AuthProvider, RepositoryError> {
        let mut tx = self.database.pool.begin().await?;

        let auth_provider = sqlx::query_as::<_, AuthProvider>(&format!(
            "INSERT INTO {} (user_id, provider, provider_user_id, access_token, refresh_token, token_expires_at) 
            VALUES ($1, $2, $3, $4, $5, $6) 
            RETURNING {}",
            Self::TABLE,
            Self::FIELDS
        ))
        .bind(&create_auth_provider.user_id)
        .bind(&create_auth_provider.provider)
        .bind(&create_auth_provider.provider_user_id)
        .bind(&create_auth_provider.access_token)
        .bind(&create_auth_provider.refresh_token)
        .bind(&create_auth_provider.token_expires_at)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                RepositoryError::UniqueViolation("refresh_token".into())
            }
            _ => RepositoryError::from(e),
        })?;

        tx.commit().await?;

        Ok(auth_provider)
    }
}
