use chrono::Utc;
use hmac::{Hmac, Mac};
use rand::{Rng, distr::Alphanumeric};
use sha2::Sha256;
use thiserror::Error;

type HmacSha256 = Hmac<Sha256>;

const TOKEN_PREFIX: &str = "vr";
const TOKEN_SEPARATOR: char = '_';
const RANDOM_PART_LENGTH: usize = 32;
const EXPIRATION_DURATION_SECONDS: i64 = 86_400; // 24 horas

#[derive(Error, Debug, PartialEq)]
pub enum HmacTokenError {
    #[error("Formato de token inválido")]
    InvalidTokenFormat,
    #[error("Timestamp inválido")]
    InvalidTimestamp,
    #[error("Chave HMAC inválida")]
    HmacKeyInvalid,
    #[error("Token expirado")]
    TokenExpired,
    #[error("Falha na geração aleatória")]
    RandomGenerationFailed,
}

pub fn generate_verification_token(user_id: &str, secret: &str) -> Result<String, HmacTokenError> {
    let random_part = generate_random_part().map_err(|_| HmacTokenError::RandomGenerationFailed)?;
    let timestamp = Utc::now().timestamp();
    let hmac_part = compute_hmac(user_id, timestamp, secret)?;

    Ok(format!(
        "{}{}{}{}{}{}{}",
        TOKEN_PREFIX,
        TOKEN_SEPARATOR,
        random_part,
        TOKEN_SEPARATOR,
        timestamp,
        TOKEN_SEPARATOR,
        hmac_part
    ))
}

pub fn validate_token(token: &str, user_id: &str, secret: &str) -> Result<bool, HmacTokenError> {
    let current_time = Utc::now().timestamp();
    validate_token_with_current_time(token, user_id, secret, current_time)
}

fn validate_token_with_current_time(
    token: &str,
    user_id: &str,
    secret: &str,
    current_time: i64,
) -> Result<bool, HmacTokenError> {
    let parts: Vec<&str> = token.split(TOKEN_SEPARATOR).collect();
    if parts.len() != 4 || parts[0] != TOKEN_PREFIX {
        return Err(HmacTokenError::InvalidTokenFormat);
    }

    let timestamp = parse_timestamp(parts[2])?;
    check_expiration(current_time, timestamp)?;

    let expected_hmac = compute_hmac(user_id, timestamp, secret)?;
    Ok(secure_compare(&expected_hmac, parts[3]))
}

fn generate_random_part() -> Result<String, anyhow::Error> {
    let mut rng = rand::rng();
    let random_bytes: Vec<u8> = (0..RANDOM_PART_LENGTH)
        .map(|_| rng.sample(Alphanumeric))
        .collect();

    String::from_utf8(random_bytes).map_err(|_| anyhow::anyhow!("Failed to generate random bytes"))
}

fn compute_hmac(user_id: &str, timestamp: i64, secret: &str) -> Result<String, HmacTokenError> {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|_| HmacTokenError::HmacKeyInvalid)?;

    mac.update(user_id.as_bytes());
    mac.update(&timestamp.to_be_bytes());

    let result = mac.finalize();
    Ok(hex::encode(result.into_bytes()))
}

fn parse_timestamp(timestamp_str: &str) -> Result<i64, HmacTokenError> {
    timestamp_str
        .parse()
        .map_err(|_| HmacTokenError::InvalidTimestamp)
}

fn check_expiration(current_time: i64, token_time: i64) -> Result<(), HmacTokenError> {
    if current_time - token_time > EXPIRATION_DURATION_SECONDS {
        Err(HmacTokenError::TokenExpired)
    } else {
        Ok(())
    }
}

fn secure_compare(a: &str, b: &str) -> bool {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();

    if a_bytes.len() != b_bytes.len() {
        return false;
    }

    a_bytes
        .iter()
        .zip(b_bytes)
        .fold(0, |acc, (x, y)| acc | (x ^ y))
        == 0
}
