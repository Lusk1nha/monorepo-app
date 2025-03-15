use std::sync::Arc;

use axum::{Router, middleware, routing::post};

use crate::{
    api_state::AppState, controllers::user_controller::UserController,
    middlewares::auth_middleware::auth_middleware,
};

pub fn user_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/update-password", post(UserController::update_password))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state)
}
