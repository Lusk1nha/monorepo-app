use chrono::{TimeDelta, Utc};

use crate::{
    entities::email_verification_entity::CreateEmailVerification,
    environment::WebPageConfig,
    errors::email_verifications_errors::EmailVerificationsError,
    repositories::email_verifications_repository::EmailVerificationsRepository,
    utils::{hmac_token, uuid::create_uuid_v4},
};

#[derive(Clone)]
pub struct EmailVerificationsService {
    pub email_verifications_repository: EmailVerificationsRepository,

    pub secret: String,
    pub expires_at: TimeDelta,

    pub web_page_config: WebPageConfig,
}

impl EmailVerificationsService {
    pub fn new(
        email_verifications_repository: EmailVerificationsRepository,
        expires_at: TimeDelta,
        secret: String,
        web_page_config: WebPageConfig,
    ) -> Self {
        Self {
            email_verifications_repository,
            expires_at,
            secret,
            web_page_config,
        }
    }

    pub async fn create_email_verification(
        &self,
        user_id: &str,
    ) -> Result<String, EmailVerificationsError> {
        let id = create_uuid_v4();
        let token = hmac_token::generate_verification_token(user_id, &self.secret)?;

        let payload = CreateEmailVerification {
            user_id: user_id.to_string(),
            expires_at: Utc::now() + self.expires_at,
            token: token.clone(),
        };

        self.email_verifications_repository
            .store_email_verification(id, &payload)
            .await?;

        let confirmation_link =
            format!("{}?token={}", self.join_confirm_path(), token);

        Ok(confirmation_link)
    }

    pub async fn confirm_email(
        &self,
        user_id: &str,
        token: &str,
    ) -> Result<(), EmailVerificationsError> {
        self.validate_email(user_id, token).await?;

        self.email_verifications_repository
            .mark_as_used(user_id)
            .await?;

        Ok(())
    }

    async fn validate_email(
        &self,
        user_id: &str,
        token: &str,
    ) -> Result<bool, EmailVerificationsError> {
        hmac_token::validate_token(token, user_id, &self.secret)
            .map_err(|_| EmailVerificationsError::InvalidToken)?;

        let email_verification = self
            .email_verifications_repository
            .find_by_user_id(user_id)
            .await?;

        match email_verification {
            Some(ev) => {
                if ev.token != token {
                    Err(EmailVerificationsError::InvalidToken)
                } else if Utc::now() > ev.expires_at {
                    Err(EmailVerificationsError::ExpiredToken)
                } else {
                    Ok(true)
                }
            }
            None => Err(EmailVerificationsError::InvalidToken),
        }
    }

    fn join_confirm_path(&self) -> String {
        format!(
            "{}{}",
            self.web_page_config.base_url, self.web_page_config.confirm_email_path
        )
    }
}
