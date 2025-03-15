use std::sync::Arc;

use crate::{
    entities::user_entity::{CreateUser, User},
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

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, UsersError> {
        let user = self
            .users_repository
            .get_user_by_email(&email)
            .await
            .map_err(UsersError::Database)?;

        Ok(user)
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

    pub async fn update_last_login_at(&self, user_id: &str) -> Result<(), UsersError> {
        self.users_repository
            .update_last_login_at(user_id)
            .await
            .map_err(UsersError::Database)?;

        Ok(())
    }
}
