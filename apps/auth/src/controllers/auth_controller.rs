use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
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
    entities::user_entity::User,
    models::auth_model::{
        CheckEmailAvailabilityRequest, CheckEmailAvailabilityResponse, ConfirmEmailRequest,
        LoginWithCredentials, RegisterWithCredentials, RegisterWithCredentialsResponse,
        SendConfirmEmailRequest, SendConfirmEmailResponse, TokenResponse, ValidateOTPCodeRequest,
    },
};

pub struct AuthController;

impl AuthController {
    pub async fn register_with_credentials(
        State(state): State<Arc<AppState>>,
        ValidatedJson(payload): ValidatedJson<RegisterWithCredentials>,
    ) -> Result<impl IntoResponse, ErrorResponse> {
        let email = payload.email.normalize();
        let password = payload.password;

        Self::is_check_email_availability(&state, &email).await?;

        let user = state
            .auth_service
            .register_user_with_credentials(&email, &password)
            .await
            .map_err(|e| Self::service_error(e, "Error registering user"))?;

        state
            .auth_service
            .send_confirm_email(&user.id)
            .await
            .map_err(|e| Self::service_error(e, "Error sending confirmation email"))?;

        Ok(Self::build_response(
            StatusCode::CREATED,
            RegisterWithCredentialsResponse {
                user_id: user.id,
                message: "User registered, please confirm your email.".to_string(),
            },
        ))
    }

    pub async fn login_with_credentials(
        jar: CookieJar,
        State(state): State<Arc<AppState>>,
        ValidatedJson(payload): ValidatedJson<LoginWithCredentials>,
    ) -> Result<impl IntoResponse, ErrorResponse> {
        let email = payload.email.normalize();
        let password = payload.password;

        let user = Self::get_user_by_email(&state, &email).await?;
        Self::verify_credentials(&state, &user.id, &password).await?;

        let session = state
            .auth_service
            .login_with_credentials(&user.id)
            .await
            .map_err(|e| Self::service_error(e, "Error logging in"))?;

        state
            .users_service
            .update_last_login_async(&user.id)
            .await
            .map_err(|e| Self::service_error(e, "Error validating OTP"))?;

        tracing::info!("User {} logged in successfully", user.id);

        let jar =
            create_refresh_token_cookie(jar, &session.refresh_token, &session.refresh_token_exp);

        Ok((StatusCode::OK, jar, Json(TokenResponse::from(session))))
    }

    pub async fn validate_otp_code(
        State(state): State<Arc<AppState>>,
        ValidatedJson(payload): ValidatedJson<ValidateOTPCodeRequest>,
    ) -> Result<impl IntoResponse, ErrorResponse> {
        let user = Self::get_user_by_id(&state, &payload.user_id).await?;

        state
            .auth_service
            .validate_otp(&user.id, &payload.code)
            .await
            .map_err(|e| Self::service_error(e, "Error validating OTP"))?;

        Ok(StatusCode::OK)
    }

    pub async fn refresh_token(
        jar: CookieJar,
        State(state): State<Arc<AppState>>,
    ) -> Result<impl IntoResponse, ErrorResponse> {
        let refresh_token = Self::extract_refresh_token(jar)?;

        let new_session = state
            .auth_service
            .create_new_refresh_token(&refresh_token)
            .await
            .map_err(|e| Self::service_error(e, "Error refreshing token"))?;

        let jar = create_refresh_token_cookie(
            CookieJar::new(),
            &new_session.refresh_token,
            &new_session.refresh_token_exp,
        );

        Ok((StatusCode::OK, jar, Json(TokenResponse::from(new_session))))
    }

    pub async fn logout(
        jar: CookieJar,
        State(state): State<Arc<AppState>>,
    ) -> Result<impl IntoResponse, ErrorResponse> {
        let refresh_token = Self::extract_refresh_token(jar)?;

        state
            .auth_service
            .revoke_refresh_token(&refresh_token)
            .await
            .map_err(|e| Self::service_error(e, "Error revoking token"))?;

        Ok((
            StatusCode::NO_CONTENT,
            remove_refresh_token_cookie(CookieJar::new()),
        ))
    }

    pub async fn check_email_availability(
        State(state): State<Arc<AppState>>,
        ValidatedJson(payload): ValidatedJson<CheckEmailAvailabilityRequest>,
    ) -> Result<impl IntoResponse, ErrorResponse> {
        let email = payload.email.normalize();
        let is_available = state
            .users_service
            .is_email_available(&email)
            .await
            .map_err(|e| Self::service_error(e, "Error checking email"))?;

        Ok(Self::build_response(
            StatusCode::OK,
            CheckEmailAvailabilityResponse {
                email,
                is_available,
            },
        ))
    }

    pub async fn send_confirm_email(
        State(state): State<Arc<AppState>>,
        ValidatedJson(payload): ValidatedJson<SendConfirmEmailRequest>,
    ) -> Result<impl IntoResponse, ErrorResponse> {
        let user_id = payload.user_id;

        state
            .auth_service
            .send_confirm_email(&user_id)
            .await
            .map_err(|e| Self::service_error(e, "Error sending email"))?;

        Ok((
            StatusCode::OK,
            Json(SendConfirmEmailResponse {
                message: "Email sent successfully.".to_string(),
            }),
        ))
    }

    pub async fn confirm_email(
        State(state): State<Arc<AppState>>,
        ValidatedJson(payload): ValidatedJson<ConfirmEmailRequest>,
    ) -> Result<impl IntoResponse, ErrorResponse> {
        let user_id = payload.user_id;
        let token = payload.token;

        println!("user_id: {}, token: {}", user_id, token);

        state
            .auth_service
            .confirm_email(&user_id, &token)
            .await
            .map_err(|e| Self::service_error(e, "Error confirming email"))?;

        Ok((
            StatusCode::OK,
            Json(SendConfirmEmailResponse {
                message: "Email confirmed successfully.".to_string(),
            }),
        ))
    }

    // Helper methods
    async fn is_check_email_availability(
        state: &Arc<AppState>,
        email: &str,
    ) -> Result<(), ErrorResponse> {
        if state
            .users_service
            .is_email_available(email)
            .await
            .map_err(|e| Self::service_error(e, "Error checking email"))?
        {
            Ok(())
        } else {
            Err(bad_request_error("Email already exists"))
        }
    }

    async fn get_user_by_email(state: &Arc<AppState>, email: &str) -> Result<User, ErrorResponse> {
        state
            .users_service
            .get_user_by_email(email)
            .await
            .map_err(|e| Self::service_error(e, "Error finding user"))?
            .ok_or_else(|| bad_request_error("User not found"))
    }

    async fn get_user_by_id(state: &Arc<AppState>, user_id: &str) -> Result<User, ErrorResponse> {
        state
            .users_service
            .get_user_by_id(user_id)
            .await
            .map_err(|e| Self::service_error(e, "Error finding user"))?
            .ok_or_else(|| bad_request_error("User not found"))
    }

    async fn verify_credentials(
        state: &Arc<AppState>,
        user_id: &str,
        password: &str,
    ) -> Result<(), ErrorResponse> {
        let is_valid = state
            .credentials_service
            .verify_user_credentials(user_id, password)
            .await
            .map_err(|e| Self::service_error(e, "Error verifying credentials"))?;

        if is_valid {
            Ok(())
        } else {
            Err(bad_request_error("Invalid credentials"))
        }
    }

    fn extract_refresh_token(jar: CookieJar) -> Result<String, ErrorResponse> {
        jar.get(REFRESH_TOKEN_NAME)
            .map(|c| c.value().to_string())
            .ok_or_else(|| bad_request_error("Missing refresh token"))
    }

    fn build_response<T: serde::Serialize>(status: StatusCode, body: T) -> (StatusCode, Json<T>) {
        (status, Json(body))
    }

    fn service_error<E: std::fmt::Display>(error: E, context: &str) -> ErrorResponse {
        tracing::error!(error = %error, "{}", context);
        internal_server_error(context)
    }
}

trait EmailNormalizer {
    fn normalize(&self) -> String;
}

impl EmailNormalizer for String {
    fn normalize(&self) -> String {
        self.trim().to_lowercase()
    }
}
