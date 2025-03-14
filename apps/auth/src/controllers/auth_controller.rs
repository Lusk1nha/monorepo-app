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
    cookies::auth_cookies::{REFRESH_TOKEN_NAME, create_refresh_token_cookie},
    core::{
        axum_response::ValidatedJson,
        error_response::ErrorResponse,
        errors_types::{bad_request_error, internal_server_error},
    },
    models::auth_model::{
        LoginWithCredentials, LoginWithCredentialsResponse, RegisterWithCredentials,
        RegisterWithCredentialsResponse,
    },
};

pub struct AuthController;

impl AuthController {
    pub async fn register_with_credentials(
        State(state): State<Arc<AppState>>,
        ValidatedJson(payload): ValidatedJson<RegisterWithCredentials>,
    ) -> Result<impl IntoResponse, ErrorResponse> {
        let email = payload.email;
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
        jar: CookieJar,
        State(state): State<Arc<AppState>>,
        ValidatedJson(payload): ValidatedJson<LoginWithCredentials>,
    ) -> Result<impl IntoResponse, ErrorResponse> {
        let email = payload.email;
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

        let session = state
            .auth_service
            .login_with_credentials(&user, &password.as_str())
            .await
            .map_err(|e| {
                tracing::error!(
                    error = %e,
                    email = %email,
                    "Error logging in with credentials."
                );
                internal_server_error("Error logging in with credentials.")
            })?;

        let jar =
            create_refresh_token_cookie(jar, &session.refresh_token, &session.refresh_token_exp);

        let headers = HeaderMap::new();

        tracing::info!("User {} logged in successfully", user.id);

        Ok((
            StatusCode::OK,
            jar,
            headers,
            Json(LoginWithCredentialsResponse::from(session)),
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
            Json(LoginWithCredentialsResponse::from(new_session)),
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

        let jar = jar.remove(REFRESH_TOKEN_NAME);

        let headers = HeaderMap::new();

        Ok((StatusCode::NO_CONTENT, jar, headers, ()))
    }
}
