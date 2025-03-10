use crate::{
    entities::auth_provider_entity::{AuthProvider, AuthProviderCreate},
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
        user_id: &str,
        provider_type: &str,
    ) -> Result<AuthProvider, sqlx::Error> {
        let create_auth_provider_payload = AuthProviderCreate {
            id: user_id.to_string(),
            provider_type: provider_type.to_string(),
        };

        self.credentials_repository
            .create_auth_provider(&create_auth_provider_payload)
            .await
    }
}
