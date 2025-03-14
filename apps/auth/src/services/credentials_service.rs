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

    pub async fn create_credential(
        &self,
        user_id: &str,
        password: &str,
    ) -> Result<Credential, CredentialsError> {
        let id = create_uuid_v4();
        let password_hash = hash_password(&password).map_err(CredentialsError::Password)?;

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
}
