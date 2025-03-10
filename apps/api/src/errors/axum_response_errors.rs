use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use thiserror::Error;

use crate::core::error_response::ErrorResponse;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(e) => {
                let message = format!("Input validation error: [{e}]").replace('\n', ", ");
                ErrorResponse::new(message, StatusCode::BAD_REQUEST).into_response()
            }
            ServerError::AxumJsonRejection(e) => {
                let status_code = e.status();
                let message = e.to_string();
                ErrorResponse::new(message, status_code).into_response()
            }
        }
    }
}
