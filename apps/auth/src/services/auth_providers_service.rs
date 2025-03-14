use crate::{
    entities::auth_provider_entity::{AuthProvider, CreateAuthProvider},
    errors::auth_providers_errors::AuthProvidersError,
    repositories::auth_providers_repository::AuthProvidersRepository,
};

#[derive(Clone)]
pub struct AuthProvidersService {
    pub credentials_repository: AuthProvidersRepository,
}

impl AuthProvidersService {
    pub fn new(credentials_repository: AuthProvidersRepository) -> Self {
        Self {
            credentials_repository,
        }
    }

    pub async fn create_auth_provider(
        &self,
        payload: &CreateAuthProvider,
    ) -> Result<AuthProvider, AuthProvidersError> {
        self.credentials_repository
            .create_auth_provider(&payload)
            .await
            .map_err(AuthProvidersError::Database)
    }
}
