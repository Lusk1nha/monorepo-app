use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::entities::{auth_provider_entity::AuthProvider, credential_entity::Credential};

#[derive(Deserialize, Validate)]
pub struct RegisterWithCredentials {
    #[validate(
        email(message = "Email is invalid"),
        length(min = 1, message = "Email is required")
    )]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterWithCredentialsResponse {
    pub id: String,
    pub message: String,
}

impl From<Credential> for RegisterWithCredentialsResponse {
    fn from(credential: Credential) -> Self {
        Self {
            id: credential.user_id,
            message: "User registered with credentials".to_string(),
        }
    }
}

#[derive(Deserialize, Validate)]
pub struct RegisterWithProvider {
    #[serde(rename = "providerId")]
    #[validate(length(min = 1, message = "Must an id from the provider"))]
    pub provider_id: String,

    #[serde(rename = "providerType")]
    #[validate(length(min = 1, message = "Must be a valid provider"))]
    pub provider_type: String,
}

#[derive(Serialize)]
pub struct RegisterWithProviderResponse {
    pub id: String,
    pub provider_type: String,
    pub message: String,
}

impl From<AuthProvider> for RegisterWithProviderResponse {
    fn from(auth_provider: AuthProvider) -> Self {
        Self {
            id: auth_provider.id,
            provider_type: auth_provider.provider_type,
            message: "User registered with provider".to_string(),
        }
    }
}

#[derive(Deserialize, Validate)]
pub struct LoginWithCredentials {
    #[validate(
        email(message = "Email is invalid"),
        length(min = 1, message = "Email is required")
    )]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,
}
