use std::sync::Arc;

use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    api_state::AppState,
    core::{
        axum_response::ValidatedJson,
        error_response::ErrorResponse,
        errors_types::{bad_request_error, internal_server_error},
    },
    entities::user_entity::User,
    models::user_model::{
        UpdateEmailRequest, UpdateEmailResponse, UpdatePasswordRequest, UpdatePasswordResponse,
        UserResponse,
    },
    services::users_service::UsersService,
};

pub struct UserController;

impl UserController {
    async fn get_current_user(
        user_service: &UsersService,
        user_id: &str,
    ) -> Result<User, ErrorResponse> {
        user_service
            .get_user_by_id(user_id)
            .await
            .map_err(|e| {
                tracing::error!(
                    error = %e,
                    user_id = %user_id,
                    "Error getting user by id."
                );
                internal_server_error("Error getting user by id.")
            })?
            .ok_or_else(|| bad_request_error("User not exists"))
    }

    pub async fn get_user(
        State(state): State<Arc<AppState>>,
        Extension(user_id): Extension<String>,
    ) -> Result<impl IntoResponse, ErrorResponse> {
        let user = Self::get_current_user(&state.users_service, &user_id).await?;
        let dto = UserResponse::from(&user);

        Ok((StatusCode::OK, Json(dto)))
    }

    pub async fn update_email(
        State(state): State<Arc<AppState>>,
        Extension(user_id): Extension<String>,
        ValidatedJson(payload): ValidatedJson<UpdateEmailRequest>,
    ) -> Result<impl IntoResponse, ErrorResponse> {
        let new_email = payload.email.trim().to_lowercase();

        let user = Self::get_current_user(&state.users_service, &user_id).await?;

        if !state
            .users_service
            .is_email_available(&new_email)
            .await
            .map_err(|e| {
                tracing::error!(
                    error = %e,
                    user_id = %user_id,
                    "Error checking if email is available."
                );
                internal_server_error("Error checking if email is available.")
            })?
        {
            return Err(bad_request_error("Email already in use."));
        }

        state
            .users_service
            .update_email(&user.id, &new_email)
            .await
            .map_err(|e| {
                tracing::error!(
                    error = %e,
                    user_id = %user_id,
                    "Error updating user email."
                );
                internal_server_error("Error updating user email.")
            })?;

        Ok((
            StatusCode::OK,
            Json(UpdateEmailResponse {
                message: "Email updated successfully.".to_string(),
            }),
        ))
    }

    pub async fn update_password(
        State(state): State<Arc<AppState>>,
        Extension(user_id): Extension<String>,
        ValidatedJson(payload): ValidatedJson<UpdatePasswordRequest>,
    ) -> Result<impl IntoResponse, ErrorResponse> {
        let password = payload.password;
        let new_password = payload.new_password;

        let user = Self::get_current_user(&state.users_service, &user_id).await?;

        let is_valid = state
            .credentials_service
            .verify_user_credentials(&user.id, &password)
            .await
            .map_err(|e| {
                tracing::error!(
                    error = %e,
                    user_id = %user_id,
                    "Error verifying user credentials."
                );
                internal_server_error("Error verifying user credentials.")
            })?;

        if !is_valid {
            return Err(bad_request_error("Invalid password."));
        }

        state
            .credentials_service
            .update_credential(&user_id, &new_password)
            .await
            .map_err(|e| {
                tracing::error!(
                    error = %e,
                    user_id = %user_id,
                    "Error updating user password."
                );
                internal_server_error("Error updating user password.")
            })?;

        Ok((
            StatusCode::OK,
            Json(UpdatePasswordResponse {
                message: "Password updated successfully.".to_string(),
            }),
        ))
    }

    pub async fn delete_user(
        State(state): State<Arc<AppState>>,
        Extension(user_id): Extension<String>,
    ) -> Result<impl IntoResponse, ErrorResponse> {
        let user = Self::get_current_user(&state.users_service, &user_id).await?;

        state
            .users_service
            .delete_user(&user.id)
            .await
            .map_err(|e| {
                tracing::error!(
                    error = %e,
                    user_id = %user_id,
                    "Error deleting user."
                );
                internal_server_error("Error deleting user.")
            })?;

        Ok((
            StatusCode::OK,
            Json(UpdatePasswordResponse {
                message: "Account deleted successfully.".to_string(),
            }),
        ))
    }
}
