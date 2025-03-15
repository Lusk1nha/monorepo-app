use crate::{
    entities::otp_code_entity::{CreateOTPCode, OTPCode},
    errors::otp_codes_errors::OTPCodesError,
    repositories::otp_codes_repository::OTPCodesRepository,
};

#[derive(Clone)]
pub struct OTPCodesService {
    pub otp_codes_repository: OTPCodesRepository,
}

impl OTPCodesService {
    pub fn new(otp_codes_repository: OTPCodesRepository) -> Self {
        Self {
            otp_codes_repository,
        }
    }

    pub async fn find_active_user_otp(
        &self,
        user_id: &str,
    ) -> Result<Option<OTPCode>, OTPCodesError> {
        todo!()
    }

    pub async fn create_otp_code(&self, user_id: &str) -> Result<OTPCode, OTPCodesError> {
        todo!()
    }

    fn generate_code() -> Result<String, OTPCodesError> {
        todo!()
    }
}
