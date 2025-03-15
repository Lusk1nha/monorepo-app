use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::core::error_response::ErrorResponse;

pub struct NotFoundController;

impl NotFoundController {
    pub async fn not_found_route() -> impl IntoResponse {
        let response = ErrorResponse {
            status_code: StatusCode::NOT_FOUND,
            message: "Route not found, for more information check the documentation".to_string(),
        };

        (StatusCode::NOT_FOUND, Json(response))
    }
}
