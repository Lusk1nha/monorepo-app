use std::sync::Arc;

use crate::{
    database::DatabaseApp,
    environment::EnvironmentApp,
    repositories::{
        auth_providers_repository::AuthProvidersRepository, credentials_repository,
        users_repository::UsersRepository,
    },
    services::{
        auth_providers_service::AuthProvidersService, credentials_service::CredentialsService,
        users_service::UsersService,
    },
};

#[derive(Clone)]
pub struct AppState {
    #[allow(dead_code)]
    pub database: DatabaseApp,
    pub environment: EnvironmentApp,

    // Add more services here
    pub users_service: Arc<UsersService>,
    pub credentials_service: Arc<CredentialsService>,
    pub auth_providers_service: Arc<AuthProvidersService>,
}

impl AppState {
    pub fn new(
        database: DatabaseApp,
        environment: EnvironmentApp,
    ) -> Result<Arc<Self>, anyhow::Error> {
        let users_repository = UsersRepository::new(database.clone());
        let users_service = UsersService::new(users_repository);

        let credentials_repository =
            credentials_repository::CredentialsRepository::new(database.clone());
        let credentials_service = CredentialsService::new(credentials_repository);

        let auth_providers_repository = AuthProvidersRepository::new(database.clone());
        let auth_providers_service = AuthProvidersService::new(auth_providers_repository);

        Ok(Arc::new(Self {
            database,
            environment,

            users_service: Arc::new(users_service),
            credentials_service: Arc::new(credentials_service),
            auth_providers_service: Arc::new(auth_providers_service),
        }))
    }
}
