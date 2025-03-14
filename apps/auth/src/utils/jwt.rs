use crate::entities::auth_refresh_token_entity::Claims;
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

#[derive(Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub refresh_token_duration: Duration,
    pub access_token_duration: Duration,
}

pub fn encode_jwt_token(
    user_id: &str,
    config: &JwtConfig,
    expires_at: &DateTime<Utc>,
) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = expires_at.timestamp() as usize;
    let iat = Utc::now().timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp,
        iat,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )
}

pub fn decode_jwt_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}
