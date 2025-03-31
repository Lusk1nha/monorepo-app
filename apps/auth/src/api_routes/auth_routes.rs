use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{api_state::AppState, controllers::auth_controller::AuthController};

pub fn auth_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/signup", post(AuthController::register_with_credentials))
        .route(
            "/send-confirm-email",
            post(AuthController::send_confirm_email),
        )
        .route("/confirm-email", post(AuthController::confirm_email))
        .route("/signin", post(AuthController::login_with_credentials))
        .route("/validate-otp", post(AuthController::validate_otp_code))
        .route("/refresh-token", post(AuthController::refresh_token))
        .route("/logout", post(AuthController::logout))
        .route(
            "/check-email-availability",
            post(AuthController::check_email_availability),
        )
        .with_state(state)
}
