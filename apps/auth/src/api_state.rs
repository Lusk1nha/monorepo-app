use std::sync::Arc;

use chrono::TimeDelta;

use crate::{
    database::DatabaseApp,
    environment::EnvironmentApp,
    repositories::{
        auth_providers_repository::AuthProvidersRepository,
        auth_refresh_token_repository::AuthRefreshTokensRepository,
        credentials_repository::CredentialsRepository, users_repository::UsersRepository,
    },
    services::{
        auth_providers_service::AuthProvidersService,
        auth_refresh_token_service::AuthRefreshTokensService, auth_service::AuthService,
        credentials_service::CredentialsService, users_service::UsersService,
    },
    utils::jwt::JwtConfig,
};

#[derive(Clone)]
pub struct AppState {
    #[allow(dead_code)]
    pub database: DatabaseApp,
    pub environment: EnvironmentApp,

    // Services
    pub auth_service: Arc<AuthService>,
    pub auth_providers_service: Arc<AuthProvidersService>,
    pub auth_refresh_tokens_service: Arc<AuthRefreshTokensService>,
    pub users_service: Arc<UsersService>,
    pub credentials_service: Arc<CredentialsService>,
}

impl AppState {
    pub fn new(
        database: DatabaseApp,
        environment: EnvironmentApp,
    ) -> Result<Arc<Self>, anyhow::Error> {
        let users_service = Arc::new(UsersService::new(UsersRepository::new(database.clone())));

        let credentials_service = Arc::new(CredentialsService::new(CredentialsRepository::new(
            database.clone(),
        )));

        let auth_providers_service = Arc::new(AuthProvidersService::new(
            AuthProvidersRepository::new(database.clone()),
        ));

        let jwt_config = JwtConfig {
            secret: environment.jwt_secret.clone(),
            access_token_duration: TimeDelta::days(1),
            refresh_token_duration: TimeDelta::days(7),
        };

        let auth_refresh_tokens_service = Arc::new(AuthRefreshTokensService::new(
            AuthRefreshTokensRepository::new(database.clone()),
            jwt_config,
        ));

        let auth_service = Arc::new(AuthService::new(
            Arc::clone(&users_service),
            Arc::clone(&credentials_service),
            Arc::clone(&auth_refresh_tokens_service),
        ));

        Ok(Arc::new(Self {
            database,
            environment,
            
            auth_service,
            auth_providers_service,
            auth_refresh_tokens_service,
            users_service,
            credentials_service,
        }))
    }
}
