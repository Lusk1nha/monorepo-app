use chrono::Utc;
use std::sync::Arc;

use crate::{
    entities::{auth_refresh_token_entity::Session, user_entity::User},
    errors::auth_service_errors::AuthServiceError,
};

use super::{
    auth_refresh_token_service::AuthRefreshTokensService, credentials_service::CredentialsService,
    otp_codes_service::OTPCodesService, users_service::UsersService,
};

#[derive(Clone)]
pub struct AuthService {
    users_service: Arc<UsersService>,
    credentials_service: Arc<CredentialsService>,
    auth_refresh_tokens_service: Arc<AuthRefreshTokensService>,
    otp_codes_service: Arc<OTPCodesService>,
}

impl AuthService {
    pub fn new(
        users_service: Arc<UsersService>,
        credentials_service: Arc<CredentialsService>,
        auth_refresh_tokens_service: Arc<AuthRefreshTokensService>,
        otp_codes_service: Arc<OTPCodesService>,
    ) -> Self {
        Self {
            users_service,
            credentials_service,
            auth_refresh_tokens_service,
            otp_codes_service,
        }
    }

    pub async fn register_user_with_credentials(
        &self,
        email: &str,
        password: &str,
    ) -> Result<User, AuthServiceError> {
        let user: User = self.users_service.create_user(&email).await?;

        self.credentials_service
            .create_credential(&user.id, &password)
            .await?;

        Ok(user)
    }

    pub async fn login_with_credentials(
        &self,
        user: &User,
        password: &str,
    ) -> Result<Session, AuthServiceError> {
        if !self
            .credentials_service
            .verify_user_credentials(&user.id, &password)
            .await?
        {
            return Err(AuthServiceError::InvalidCredentials);
        }

        self.auth_refresh_tokens_service
            .create_session(&user.id)
            .await
            .map_err(|_| AuthServiceError::CreateAuthRefreshTokenError)
    }

    pub async fn login_with_otp(
        &self,
        user_id: &str,
        code: &str,
    ) -> Result<Session, AuthServiceError> {
        let user = self.users_service.get_user_by_id(&user_id).await?;

        if user.is_none() {
            return Err(AuthServiceError::UserNotFound);
        }

        let user = user.unwrap();

        self.otp_codes_service
            .validate_otp_code(&user.id, &code)
            .await?;

        self.auth_refresh_tokens_service
            .create_session(&user.id)
            .await
            .map_err(|_| AuthServiceError::CreateAuthRefreshTokenError)
    }

    pub async fn create_new_refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<Session, AuthServiceError> {
        let stored_session = self
            .auth_refresh_tokens_service
            .find_session_by_hash(&refresh_token)
            .await
            .map_err(|_| AuthServiceError::CreateAuthRefreshTokenError)?;

        if stored_session.is_none() {
            return Err(AuthServiceError::RefreshTokenNotFound);
        }

        let stored_session = stored_session.unwrap();

        if stored_session.expires_at < Utc::now() {
            return Err(AuthServiceError::RefreshTokenNotFound);
        }

        self.auth_refresh_tokens_service
            .update_session(&stored_session)
            .await
            .map_err(|_| AuthServiceError::CreateAuthRefreshTokenError)
    }

    pub async fn revoke_refresh_token(&self, refresh_token: &str) -> Result<(), AuthServiceError> {
        let stored_session = self
            .auth_refresh_tokens_service
            .find_session_by_hash(&refresh_token)
            .await
            .map_err(|_| AuthServiceError::RevokeRefreshTokenError)?;

        if stored_session.is_none() {
            return Err(AuthServiceError::RefreshTokenNotFound);
        }

        let stored_session = stored_session.unwrap();

        self.auth_refresh_tokens_service
            .revoke_session_by_hash(&stored_session.id)
            .await
            .map_err(|_| AuthServiceError::RevokeRefreshTokenError)
    }
}
