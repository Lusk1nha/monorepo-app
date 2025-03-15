use crate::{
    entities::auth_refresh_token_entity::{
        AuthRefreshToken, CreateAuthRefreshToken, Session, UpdateAuthRefreshToken,
    },
    errors::auth_refresh_tokens_errors::AuthRefreshTokensError,
    repositories::auth_refresh_token_repository::AuthRefreshTokensRepository,
    utils::{
        jwt::{JwtConfig, encode_jwt_token},
        uuid::create_uuid_v4,
    },
};

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::Utc;
use rand_core::{OsRng, TryRngCore};
use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Clone)]
pub struct AuthRefreshTokensService {
    repository: AuthRefreshTokensRepository,
    jwt_config: JwtConfig,
}

#[derive(Debug, Error)]
pub enum TokenGenerationError {
    #[error("Falha na geração do token")]
    TokenGenerationFailed,
    #[error("Configuração JWT inválida")]
    InvalidJwtConfig,
}

impl AuthRefreshTokensService {
    pub fn new(repository: AuthRefreshTokensRepository, jwt_config: JwtConfig) -> Self {
        Self {
            repository,
            jwt_config,
        }
    }

    pub async fn create_session(&self, user_id: &str) -> Result<Session, AuthRefreshTokensError> {
        let refresh_token = self.generate_secure_refresh_token()?;
        let token_hash = self.hash_refresh_token(&refresh_token)?;

        let refresh_expires_at = Utc::now() + self.jwt_config.refresh_token_duration;
        let access_expires_at = Utc::now() + self.jwt_config.access_token_duration;

        let access_token = encode_jwt_token(user_id, &self.jwt_config, &access_expires_at)
            .map_err(|e| AuthRefreshTokensError::JwtGenerationError(e.into()))?;

        let payload = CreateAuthRefreshToken {
            user_id: user_id.to_string(),
            token_hash,
            expires_at: refresh_expires_at,
        };

        self.repository
            .store_refresh_token(&create_uuid_v4(), &payload)
            .await?;

        Ok(Session {
            access_token,
            refresh_token,
            access_token_exp: access_expires_at,
            refresh_token_exp: refresh_expires_at,
        })
    }

    pub async fn find_session_by_hash(
        &self,
        refresh_token: &str,
    ) -> Result<Option<AuthRefreshToken>, AuthRefreshTokensError> {
        let token_hash = self.hash_refresh_token(refresh_token)?;

        self.repository
            .find_by_hash(&token_hash)
            .await
            .map_err(AuthRefreshTokensError::Database)
    }

    pub async fn update_session(
        &self,
        actual_token: &AuthRefreshToken,
    ) -> Result<Session, AuthRefreshTokensError> {
        let new_refresh_token = self.generate_secure_refresh_token()?;
        let new_token_hash = self.hash_refresh_token(&new_refresh_token)?;

        let new_refresh_expires_at = Utc::now() + self.jwt_config.refresh_token_duration;
        let access_expires_at = Utc::now() + self.jwt_config.access_token_duration;

        let access_token =
            encode_jwt_token(&actual_token.user_id, &self.jwt_config, &access_expires_at)
                .map_err(|e| AuthRefreshTokensError::JwtGenerationError(e.into()))?;

        let payload = UpdateAuthRefreshToken {
            token_hash: new_token_hash,
            expires_at: new_refresh_expires_at,
        };

        self.repository
            .update_refresh_token(&actual_token.id, &payload)
            .await?;

        Ok(Session {
            access_token,
            refresh_token: new_refresh_token,
            access_token_exp: access_expires_at,
            refresh_token_exp: new_refresh_expires_at,
        })
    }

    pub async fn revoke_session_by_hash(&self, id: &str) -> Result<(), AuthRefreshTokensError> {
        let current_time = Utc::now();

        self.repository
            .revoke_refresh_token(&id, current_time)
            .await
            .map_err(AuthRefreshTokensError::Database)
    }

    fn generate_secure_refresh_token(&self) -> Result<String, AuthRefreshTokensError> {
        let mut bytes = [0u8; 64]; // 512 bits de entropia
        let _ = OsRng.try_fill_bytes(&mut bytes);
        Ok(URL_SAFE_NO_PAD.encode(bytes))
    }

    fn hash_refresh_token(&self, token: &str) -> Result<String, AuthRefreshTokensError> {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        Ok(URL_SAFE_NO_PAD.encode(hasher.finalize()))
    }
}
