use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct UpdatePasswordRequest {
    #[validate(length(min = 8))]
    pub password: String,

    #[serde(rename = "newPassword")]
    #[validate(length(min = 8))]
    pub new_password: String,
}

#[derive(Serialize)]
pub struct UpdatePasswordResponse {
    pub message: String,
}
