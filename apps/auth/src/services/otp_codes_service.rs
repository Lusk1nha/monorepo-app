use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};

use chrono::{TimeDelta, Utc};
use rand::TryRngCore;
use rand_core::OsRng;
use totp_rs::{Algorithm, Secret, TOTP};

use crate::{
    entities::otp_code_entity::{CreateOTPCode, OTPCode},
    errors::otp_codes_errors::OTPCodesError,
    repositories::otp_codes_repository::OTPCodesRepository,
};

#[derive(Clone)]
pub struct OTPCodesService {
    pub otp_codes_repository: OTPCodesRepository,
    pub expires_at: TimeDelta,
}

impl OTPCodesService {
    pub fn new(otp_codes_repository: OTPCodesRepository, expires_at: TimeDelta) -> Self {
        Self {
            otp_codes_repository,
            expires_at,
        }
    }

    async fn find_active_user_otp(&self, user_id: &str) -> Result<Option<OTPCode>, OTPCodesError> {
        self.otp_codes_repository
            .find_active_user_otp(user_id)
            .await
            .map_err(OTPCodesError::Database)
    }

    pub async fn validate_otp_code(&self, user_id: &str, code: &str) -> Result<(), OTPCodesError> {
        let otp = self.find_active_user_otp(user_id).await?;

        if otp.is_none() {
            return Err(OTPCodesError::OTPNotFound);
        }

        let otp = otp.unwrap();

        if otp.code != code {
            return Err(OTPCodesError::InvalidCode);
        }

        self.otp_codes_repository
            .use_otp_code(&otp.id, user_id)
            .await?;

        Ok(())
    }

    pub async fn create_otp_code(
        &self,
        user_id: &str,
        secret: &str,
    ) -> Result<OTPCode, OTPCodesError> {
        let code = self
            .generate_code_from_secret(&secret)
            .await
            .map_err(|_| OTPCodesError::GenerateCode)?;

        let expires_at = Utc::now() + self.expires_at;

        let payload = CreateOTPCode {
            user_id: user_id.to_string(),
            code,
            expires_at,
        };

        self.otp_codes_repository
            .create_otp_code(&payload)
            .await
            .map_err(|_| OTPCodesError::CreateOTPCode)
    }

    pub async fn generate_secure_otp_secret(&self) -> Result<String, OTPCodesError> {
        let mut bytes = [0u8; 32];
        let _ = OsRng.try_fill_bytes(&mut bytes);

        Ok(URL_SAFE_NO_PAD.encode(bytes))
    }

    async fn generate_code_from_secret(&self, secret: &str) -> Result<String, OTPCodesError> {
        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            Secret::Raw(secret.as_bytes().to_vec()).to_bytes().unwrap(),
        )
        .unwrap();

        let code = totp
            .generate_current()
            .map_err(|_| OTPCodesError::GenerateCode)?;

        Ok(code)
    }
}
