use bcrypt::DEFAULT_COST;

use crate::{
    entities::credential_entity::{CreateCredential, Credential},
    errors::credentials_errors::CredentialsError,
    repositories::credentials_repository::CredentialsRepository,
    utils::{password::hash_password, uuid::create_uuid_v4},
};

#[derive(Clone)]
pub struct CredentialsService {
    pub credentials_repository: CredentialsRepository,
}

impl CredentialsService {
    pub fn new(credentials_repository: CredentialsRepository) -> Self {
        Self {
            credentials_repository,
        }
    }

    pub async fn get_credential_with_email(
        &self,
        email: &str,
    ) -> Result<Option<Credential>, CredentialsError> {
        let credential = self
            .credentials_repository
            .get_credential_with_email(&email)
            .await
            .map_err(CredentialsError::Database)?;

        Ok(credential)
    }

    pub async fn create_credential(
        &self,
        user_id: &str,
        email: &str,
        password: &str,
    ) -> Result<Credential, CredentialsError> {
        let id = create_uuid_v4();
        let password_hash =
            hash_password(&password, DEFAULT_COST).map_err(CredentialsError::Password)?;

        let create_credential_payload = CreateCredential {
            id,
            user_id: user_id.to_string(),
            email: email.to_string(),
            password_hash,
        };

        let credential = self.credentials_repository
            .create_credential_transaction(&create_credential_payload)
            .await
            .map_err(CredentialsError::Database)?;

        Ok(credential)
    }
}
