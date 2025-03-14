use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{api_state::AppState, controllers::auth_controller::AuthController};

pub fn auth_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/register-with-credentials",
            post(AuthController::register_with_credentials),
        )
        .route(
            "/login-with-credentials",
            post(AuthController::login_with_credentials),
        )
        .route("/refresh-token", post(AuthController::refresh_token))
        .route("/logout", post(AuthController::logout))
        .with_state(state)
}
