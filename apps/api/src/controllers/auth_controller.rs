use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    api_state::AppState,
    core::{
        axum_response::ValidatedJson,
        error_response::ErrorResponse,
        errors_types::{bad_request_error, internal_server_error},
    },
    models::auth_model::{RegisterWithCredentials, RegisterWithCredentialsResponse},
};

pub struct AuthController;

impl AuthController {
    pub async fn register_with_credentials(
        State(state): State<Arc<AppState>>,
        ValidatedJson(payload): ValidatedJson<RegisterWithCredentials>,
    ) -> Result<impl IntoResponse, ErrorResponse> {
        let email = payload.email;
        let password = payload.password;

        match state.users_service.get_user_by_email(&email).await {
            Ok(Some(_)) => {
                return Err(bad_request_error("User with this email already exists."));
            }
            Ok(None) => {}
            Err(e) => {
                tracing::error!("Error checking if user exists: {:?}", e);
                return Err(internal_server_error("Error checking if user exists."));
            }
        }

        let created_user = state
            .users_service
            .create_user_credentials(&email)
            .await
            .map_err(|e| {
                tracing::error!("Error creating user: {:?}", e);
                internal_server_error("Error creating user.")
            })?;

        let created_credential = state
            .credentials_service
            .create_credential(&created_user.id, &password)
            .await
            .map_err(|e| {
                tracing::error!("Error creating credential: {:?}", e);
                internal_server_error("Error creating credential.")
            })?;

        let server_response = RegisterWithCredentialsResponse::from(created_credential);

        Ok((StatusCode::CREATED, Json(server_response)))
    }
}
