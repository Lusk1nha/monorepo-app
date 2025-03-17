use std::sync::Arc;

use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;

use crate::{
    api_state::AppState,
    cookies::auth_cookies::{
        REFRESH_TOKEN_NAME, create_refresh_token_cookie, remove_refresh_token_cookie,
    },
    core::{
        axum_response::ValidatedJson,
        error_response::ErrorResponse,
        errors_types::{bad_request_error, internal_server_error},
    },
    models::auth_model::{
        CheckEmailAvailabilityRequest, CheckEmailAvailabilityResponse, LoginWithCredentials,
        LoginWithCredentialsResponse, RegisterWithCredentials, RegisterWithCredentialsResponse,
        TokenResponse, ValidateOTPCodeRequest,
    },
};

pub struct AuthController;

impl AuthController {
    pub async fn register_with_credentials(
        State(state): State<Arc<AppState>>,
        ValidatedJson(payload): ValidatedJson<RegisterWithCredentials>,
    ) -> Result<impl IntoResponse, ErrorResponse> {
        let email = payload.email.trim().to_lowercase();
        let password = payload.password;

        let existing_user = state
            .users_service
            .get_user_by_email(&email)
            .await
            .map_err(|e| {
                tracing::error!(
                    error = %e,
                    email = %email,
                    "Error checking existing user."
                );
                internal_server_error("Error checking existing user.")
            })?;

        if existing_user.is_some() {
            return Err(bad_request_error("User with this email already exists."));
        }

        let user = state
            .auth_service
            .register_user_with_credentials(&email, &password)
            .await
            .map_err(|e| {
                tracing::error!(
                    error = %e,
                    email = %email,
                    "Error registering user with credentials."
                );
                internal_server_error("Error registering user with credentials.")
            })?;

        Ok((
            StatusCode::CREATED,
            Json(RegisterWithCredentialsResponse::from(user)),
        ))
    }

    pub async fn login_with_credentials(
        State(state): State<Arc<AppState>>,
        ValidatedJson(payload): ValidatedJson<LoginWithCredentials>,
    ) -> Result<impl IntoResponse, ErrorResponse> {
        let email = payload.email.trim().to_lowercase();
        let password = payload.password;

        let existing_user = state
            .users_service
            .get_user_by_email(&email)
            .await
            .map_err(|e| {
                tracing::error!(
                    error = %e,
                    email = %email,
                    "Error checking existing user."
                );
                internal_server_error("Error checking existing user.")
            })?;

        if existing_user.is_none() {
            return Err(bad_request_error("User with this email does not exist."));
        }

        let user = existing_user.unwrap();

        if !state
            .credentials_service
            .verify_user_credentials(&user.id, &password)
            .await
            .map_err(|e| {
                tracing::error!(
                    error = %e,
                    email = %email,
                    "Error verifying user credentials."
                );
                internal_server_error("Error verifying user credentials.")
            })?
        {
            return Err(bad_request_error("Invalid credentials."));
        }

        state
            .otp_service
            .create_otp_code(&user.id, &user.otp_secret)
            .await
            .map_err(|e| {
                tracing::error!(
                    error = %e,
                    user_id = %user.id,
                    "Error creating OTP code."
                );
                internal_server_error("Error creating OTP code.")
            })?;

        Self::sync_update_last_login(&state, &user.id).await;

        tracing::info!("User {} logged in successfully", user.id);

        Ok((
            StatusCode::OK,
            Json(LoginWithCredentialsResponse {
                user_id: user.id,
                message: "Logged in successfully, OTP code sent to your email.".to_string(),
            }),
        ))
    }

    pub async fn validate_otp_code(
        jar: CookieJar,
        State(state): State<Arc<AppState>>,
        ValidatedJson(payload): ValidatedJson<ValidateOTPCodeRequest>,
    ) -> Result<impl IntoResponse, ErrorResponse> {
        let code = payload.code.trim();
        let user_id = payload.user_id.trim();

        let session = state
            .auth_service
            .login_with_otp(&user_id, &code)
            .await
            .map_err(|e| {
                tracing::error!(
                    error = %e,
                    user_id = %user_id,
                    "Error logging in with OTP."
                );
                internal_server_error("Error logging in with OTP.")
            })?;

        let headers = HeaderMap::new();

        let jar =
            create_refresh_token_cookie(jar, &session.refresh_token, &session.refresh_token_exp);

        Ok((
            StatusCode::OK,
            jar,
            headers,
            Json(TokenResponse::from(session)),
        ))
    }

    pub async fn refresh_token(
        jar: CookieJar,
        State(state): State<Arc<AppState>>,
    ) -> Result<impl IntoResponse, ErrorResponse> {
        let refresh_token = jar
            .get(REFRESH_TOKEN_NAME)
            .map(|c| c.value().to_string())
            .ok_or_else(|| bad_request_error("Refresh token not found."))?;

        let new_session = state
            .auth_service
            .create_new_refresh_token(&refresh_token)
            .await
            .map_err(|e| {
                tracing::error!(
                    error = %e,
                    "Error creating new refresh token."
                );
                internal_server_error("Error creating new refresh token.")
            })?;

        let jar = create_refresh_token_cookie(
            jar,
            &new_session.refresh_token,
            &new_session.refresh_token_exp,
        );

        let headers = HeaderMap::new();

        Ok((
            StatusCode::OK,
            jar,
            headers,
            Json(TokenResponse::from(new_session)),
        ))
    }

    pub async fn logout(
        jar: CookieJar,
        State(state): State<Arc<AppState>>,
    ) -> Result<impl IntoResponse, ErrorResponse> {
        let refresh_token = jar
            .get(REFRESH_TOKEN_NAME)
            .map(|c| c.value().to_string())
            .ok_or_else(|| bad_request_error("Refresh token not found."))?;

        state
            .auth_service
            .revoke_refresh_token(&refresh_token)
            .await
            .map_err(|e| {
                tracing::error!(
                    error = %e,
                    "Error revoking refresh token."
                );
                internal_server_error("Error revoking refresh token.")
            })?;

        let jar = remove_refresh_token_cookie(jar);

        let headers = HeaderMap::new();

        Ok((StatusCode::NO_CONTENT, jar, headers, ()))
    }

    pub async fn check_email_availability(
        State(state): State<Arc<AppState>>,
        ValidatedJson(payload): ValidatedJson<CheckEmailAvailabilityRequest>,
    ) -> Result<impl IntoResponse, ErrorResponse> {
        let email = payload.email.trim().to_lowercase();

        let is_available = state
            .users_service
            .is_email_available(&email)
            .await
            .map_err(|e| {
                tracing::error!(
                    error = %e,
                    email = %email,
                    "Error checking email availability."
                );
                internal_server_error("Error checking email availability.")
            })?;

        Ok((
            StatusCode::OK,
            Json(CheckEmailAvailabilityResponse {
                email,
                is_available,
            }),
        ))
    }

    async fn sync_update_last_login(state: &Arc<AppState>, user_id: &str) {
        let state = state.clone();
        let user_id = user_id.to_string();

        tokio::spawn(async move {
            if let Err(e) = state.users_service.update_last_login_at(&user_id).await {
                tracing::error!("Failed to update last login for user {}: {}", user_id, e);
            }
        });
    }
}
