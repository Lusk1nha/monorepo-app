use axum::{Extension, Router};
use std::sync::Arc;
use tracing::info;

use crate::{
    api_routes::{auth_routes::auth_routes, root_routes::root_routes},
    api_state::AppState,
    core::cors::configure_cors,
};

const API_NEST_PATH: &str = "/api";
const AUTH_NEST_PATH: &str = "/auth";

pub fn create_api_routes(state: Arc<AppState>) -> Router {
    info!("Configuring API routes");

    let cors = configure_cors();
    let api_routes = api_routes(state);

    Router::new().nest(API_NEST_PATH, api_routes).layer(cors)
}

fn api_routes(state: Arc<AppState>) -> Router {
    info!("Nesting routes under {}", API_NEST_PATH);

    let root_routes = root_routes(state.clone());
    let auth_routes = auth_routes(state.clone());

    root_routes
        .nest(AUTH_NEST_PATH, auth_routes)
        .layer(Extension(state))
}
