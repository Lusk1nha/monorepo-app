use crate::{
    entities::credential_entity::{CreateCredential, Credential, UpdateCredential},
    errors::credentials_errors::CredentialsError,
    repositories::credentials_repository::CredentialsRepository,
    utils::uuid::create_uuid_v4,
};

use bcrypt::{DEFAULT_COST, hash, verify};

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

    pub async fn get_credential_by_user_id(
        &self,
        user_id: &str,
    ) -> Result<Option<Credential>, CredentialsError> {
        let credential = self
            .credentials_repository
            .find_by_id(&user_id)
            .await
            .map_err(CredentialsError::Database)?;

        Ok(credential)
    }

    pub async fn verify_user_credentials(
        &self,
        user_id: &str,
        password: &str,
    ) -> Result<bool, CredentialsError> {
        let credential = self.get_credential_by_user_id(user_id).await?;

        match credential {
            Some(credential) => {
                let is_valid = self
                    .verify_hash_password(password, &credential.password_hash)
                    .map_err(CredentialsError::Password)?;

                Ok(is_valid)
            }
            None => Ok(false),
        }
    }

    pub async fn create_credential(
        &self,
        user_id: &str,
        password: &str,
    ) -> Result<Credential, CredentialsError> {
        let id = create_uuid_v4();
        let password_hash = self
            .hash_password(password)
            .map_err(CredentialsError::Password)?;

        let payload = CreateCredential {
            user_id: user_id.to_string(),
            password_hash,
            algorithm: "bcrypt".to_string(),
        };

        let credential = self
            .credentials_repository
            .create_credential(&id, &payload)
            .await
            .map_err(CredentialsError::Database)?;

        Ok(credential)
    }

    pub async fn update_credential(
        &self,
        user_id: &str,
        password: &str,
    ) -> Result<Credential, CredentialsError> {
        let password_hash = self
            .hash_password(password)
            .map_err(CredentialsError::Password)?;

        let payload = UpdateCredential {
            password_hash,
            algorithm: "bcrypt".to_string(),
        };

        let credential = self
            .credentials_repository
            .update_credential(user_id, &payload)
            .await
            .map_err(CredentialsError::Database)?;

        Ok(credential)
    }

    pub fn verify_hash_password(
        &self,
        password: &str,
        hash: &str,
    ) -> Result<bool, bcrypt::BcryptError> {
        verify(password, hash)
    }

    fn hash_password(&self, password: &str) -> Result<String, bcrypt::BcryptError> {
        hash(password, DEFAULT_COST)
    }
}
