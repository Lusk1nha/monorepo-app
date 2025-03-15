use std::sync::Arc;

use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    api_state::AppState,
    core::{
        axum_response::ValidatedJson,
        error_response::ErrorResponse,
        errors_types::{bad_request_error, internal_server_error},
    },
    models::user_model::{UpdatePasswordRequest, UpdatePasswordResponse},
};

pub struct UserController;

impl UserController {
    pub async fn update_password(
        State(state): State<Arc<AppState>>,
        Extension(user_id): Extension<String>,
        ValidatedJson(payload): ValidatedJson<UpdatePasswordRequest>,
    ) -> Result<impl IntoResponse, ErrorResponse> {
        let password = payload.password;
        let new_password = payload.new_password;

        if !state
            .credentials_service
            .verify_user_credentials(&user_id, &password)
            .await
            .map_err(|e| {
                tracing::error!(
                    error = %e,
                    user_id = %user_id,
                    "Error verifying user credentials."
                );
                internal_server_error("Error verifying user credentials.")
            })?
        {
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
}
