use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::core::error_response::ErrorResponse;

const NOT_FOUND_MESSAGE: &str = "Route not found, for more information check the documentation";
const NOT_FOUND_STATUS: StatusCode = StatusCode::NOT_FOUND;

pub struct NotFoundController;

impl NotFoundController {
    pub async fn not_found_route() -> impl IntoResponse {
        let response = ErrorResponse::new(NOT_FOUND_MESSAGE.to_string(), NOT_FOUND_STATUS);
        (NOT_FOUND_STATUS, Json(response))
    }
}
