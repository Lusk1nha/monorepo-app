use std::sync::Arc;

use crate::{
    entities::user_entity::{CreateUser, UpdateUser, User},
    errors::users_errors::UsersError,
    repositories::users_repository::UsersRepository,
    utils::uuid::create_uuid_v4,
};

use super::otp_codes_service::OTPCodesService;

#[derive(Clone)]
pub struct UsersService {
    pub users_repository: UsersRepository,
    pub otp_codes_service: Arc<OTPCodesService>,
}

impl UsersService {
    pub fn new(users_repository: UsersRepository, otp_codes_service: Arc<OTPCodesService>) -> Self {
        Self {
            users_repository,
            otp_codes_service,
        }
    }

    pub async fn get_user_by_id(&self, user_id: &str) -> Result<Option<User>, UsersError> {
        let user = self
            .users_repository
            .get_user_by_id(&user_id)
            .await
            .map_err(UsersError::Database)?;

        Ok(user)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, UsersError> {
        let user = self
            .users_repository
            .get_user_by_email(&email)
            .await
            .map_err(UsersError::Database)?;

        Ok(user)
    }

    pub async fn is_email_available(&self, email: &str) -> Result<bool, UsersError> {
        let exists = self.get_user_by_email(email).await?.is_some();
        Ok(!exists)
    }

    pub async fn create_user(&self, email: &str) -> Result<User, UsersError> {
        let id = create_uuid_v4();
        let otp_secret = self
            .otp_codes_service
            .generate_secure_otp_secret()
            .await
            .map_err(UsersError::OTPSecret)?;

        let payload = CreateUser {
            email: email.into(),
            name: None,
            image: None,

            otp_secret,
        };

        let user = self
            .users_repository
            .create_user(&id, &payload)
            .await
            .map_err(UsersError::Database)?;

        Ok(user)
    }

    pub async fn update_email(&self, user_id: &str, email: &str) -> Result<User, UsersError> {
        let payload = UpdateUser {
            email: Some(email.into()),

            name: None,
            image: None,
            is_2fa_enabled: None,
            is_email_verified: None,
            otp_secret: None,
        };

        let user = self
            .users_repository
            .update_user_properties(user_id, payload)
            .await
            .map_err(UsersError::Database)?;

        Ok(user)
    }

    pub async fn update_last_login_async(&self, user_id: &str) -> Result<(), UsersError> {
        let repository = self.users_repository.clone();
        let user_id = user_id.to_string();

        tokio::spawn(async move {
            if let Err(e) = repository.update_last_login_at(&user_id).await {
                tracing::error!("Failed to update last login for user {}: {}", user_id, e);
            }
        });

        Ok(())
    }

    pub async fn update_email_verified(&self, user_id: &str) -> Result<(), UsersError> {
        let repository = self.users_repository.clone();
        let user_id = user_id.to_string();

        tokio::spawn(async move {
            if let Err(e) = repository.update_email_verified(&user_id).await {
                tracing::error!("Failed to update last login for user {}: {}", user_id, e);
            }
        });

        Ok(())
    }

    pub async fn delete_user(&self, user_id: &str) -> Result<(), UsersError> {
        self.users_repository
            .delete_user(user_id)
            .await
            .map_err(UsersError::Database)?;

        Ok(())
    }
}
