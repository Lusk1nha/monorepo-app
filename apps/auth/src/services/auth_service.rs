use chrono::Utc;
use mail_service::{MailService, errors::MailServiceError};
use std::sync::Arc;

use crate::{
    emails::auth_emails::AuthEmailType,
    entities::{auth_refresh_token_entity::Session, user_entity::User},
    errors::auth_service_errors::AuthServiceError,
};

use super::{
    auth_refresh_token_service::AuthRefreshTokensService, credentials_service::CredentialsService,
    email_verifications_service::EmailVerificationsService, otp_codes_service::OTPCodesService,
    users_service::UsersService,
};

#[derive(Clone)]
pub struct AuthService {
    users_service: Arc<UsersService>,
    credentials_service: Arc<CredentialsService>,
    auth_refresh_tokens_service: Arc<AuthRefreshTokensService>,
    otp_codes_service: Arc<OTPCodesService>,
    email_verifications_service: Arc<EmailVerificationsService>,

    mail_service: Arc<MailService>,
}

impl AuthService {
    pub fn new(
        users_service: Arc<UsersService>,
        credentials_service: Arc<CredentialsService>,
        auth_refresh_tokens_service: Arc<AuthRefreshTokensService>,
        otp_codes_service: Arc<OTPCodesService>,
        email_verifications_service: Arc<EmailVerificationsService>,
        mail_service: Arc<MailService>,
    ) -> Self {
        Self {
            users_service,
            credentials_service,
            auth_refresh_tokens_service,
            otp_codes_service,
            email_verifications_service,

            mail_service,
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

    pub async fn send_otp_code(&self, from: &str, user: &User) -> Result<(), AuthServiceError> {
        let otp_code = self
            .otp_codes_service
            .create_otp_code(&user.id, &user.otp_secret)
            .await?;

        Self::queue_otp_code_email(&self, from, &user.email, &otp_code.code).await?;

        Ok(())
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

    pub async fn send_confirm_email(&self, user_id: &str) -> Result<(), AuthServiceError> {
        let user = self.get_user_by_id(&user_id).await?;

        let confirmation_link = self
            .email_verifications_service
            .create_email_verification(&user.id)
            .await?;

        self.queue_confirm_email(
            "personalfeedbackapptest@gmail.com",
            &user.email,
            confirmation_link,
        )
        .await?;

        Ok(())
    }

    pub async fn confirm_email(&self, user_id: &str, token: &str) -> Result<(), AuthServiceError> {
        let user = self.get_user_by_id(&user_id).await?;

        self.email_verifications_service
            .confirm_email_verification(&user.id, token)
            .await?;

        self.users_service.update_email_verified(&user.id).await?;

        Ok(())
    }

    async fn queue_otp_code_email(
        &self,
        from: &str,
        user_email: &str,
        otp_code: &str,
    ) -> Result<(), MailServiceError> {
        let email_type = AuthEmailType::OtpCode {
            from: from.to_string(),
            to: user_email.to_string(),
            code: otp_code.to_string(),
        };

        let email_request = email_type.build_request();

        self.mail_service.queue_email(email_request).await?;

        Ok(())
    }

    async fn queue_confirm_email(
        &self,
        from: &str,
        user_email: &str,
        confirmation_link: String,
    ) -> Result<(), MailServiceError> {
        let email_type = AuthEmailType::ConfirmEmail {
            from: from.to_string(),
            to: user_email.to_string(),
            confirmation_link,
        };

        let email_request = email_type.build_request();

        self.mail_service.queue_email(email_request).await?;

        Ok(())
    }

    async fn get_user_by_id(&self, user_id: &str) -> Result<User, AuthServiceError> {
        let user = self.users_service.get_user_by_id(&user_id).await?;

        if user.is_none() {
            return Err(AuthServiceError::UserNotFound);
        }

        let user = user.unwrap();

        Ok(user)
    }
}
