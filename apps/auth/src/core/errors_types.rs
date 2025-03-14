use axum::http::StatusCode;

use super::error_response::ErrorResponse;

pub fn internal_server_error(message: impl Into<String>) -> ErrorResponse {
    ErrorResponse {
        message: message.into(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub fn bad_request_error(message: impl Into<String>) -> ErrorResponse {
    ErrorResponse::new(message.into(), StatusCode::BAD_REQUEST)
}

pub fn unauthorized_error(message: impl Into<String>) -> ErrorResponse {
    ErrorResponse::new(message.into(), StatusCode::UNAUTHORIZED)
}

pub fn not_found_error(message: impl Into<String>) -> ErrorResponse {
    ErrorResponse::new(message.into(), StatusCode::NOT_FOUND)
}
