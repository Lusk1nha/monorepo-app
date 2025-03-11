use crate::{
    database::DatabaseApp,
    entities::credential_entity::{CreateCredential, Credential},
};

#[derive(Clone)]
pub struct CredentialsRepository {
    database: DatabaseApp,
}

const CREDENTIALS_FIELDS: &str = "id, user_id, password_hash, alghorithm, created_at, updated_at";

impl CredentialsRepository {
    pub fn new(database: DatabaseApp) -> Self {
        Self { database }
    }

    pub async fn get_credential_with_user_id(
        &self,
        user_id: &str,
    ) -> Result<Option<Credential>, sqlx::Error> {
        let credential: Option<Credential> = sqlx::query_as::<_, Credential>(&format!(
            "SELECT {CREDENTIALS_FIELDS} FROM credentials WHERE user_id = $1"
        ))
        .bind(&user_id)
        .fetch_optional(&self.database.pool)
        .await?;

        Ok(credential)
    }

    pub async fn create_credential_transaction(
        &self,
        create_credential: &CreateCredential,
    ) -> Result<Credential, sqlx::Error> {
        let mut tx = self.database.pool.begin().await?;

        sqlx::query(
            "INSERT INTO credentials (id, user_id, password_hash, alghorithm) VALUES ($1, $2, $3, $4)",
        )
        .bind(&create_credential.id)
        .bind(&create_credential.user_id)
        .bind(&create_credential.password_hash)
        .bind(&create_credential.alghorithm)
        .execute(&mut *tx)
        .await?;

        let credential: Credential = sqlx::query_as::<_, Credential>(&format!(
            "SELECT {CREDENTIALS_FIELDS} FROM credentials WHERE id = $1"
        ))
        .bind(&create_credential.id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(credential)
    }
}
