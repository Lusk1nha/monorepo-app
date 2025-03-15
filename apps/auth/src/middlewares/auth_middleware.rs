use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Request, State},
    http::{HeaderMap, HeaderValue, header::AUTHORIZATION},
    middleware::Next,
    response::IntoResponse,
};

use crate::{
    api_state::AppState,
    core::{error_response::ErrorResponse, errors_types::unauthorized_error},
    utils::jwt::decode_jwt_token,
};

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, ErrorResponse> {
    let headers = req.headers();

    let secret = state.environment.jwt_secret.clone();
    let token = extract_bearer_token(headers)
        .ok_or_else(|| unauthorized_error("Token must be provided in the Authorization header"))?;

    let claims = decode_jwt_token(&token, &secret)
        .map_err(|_| unauthorized_error("Session not found or expired"))?;

    let user_id = claims.sub.clone();

    req.extensions_mut().insert(claims);
    req.extensions_mut().insert(user_id);

    Ok(next.run(req).await)
}

/**
 * Extract the bearer token from the headers
 * - Doesn't need to have bearer in the token
 * - The token is expected to be in the Authorization header
 * - The token is expected to be a Bearer token
 * - If the token is not a Bearer token, it will be concatenated with the Bearer prefix
 * - Returns None if the token is not found
 */
fn extract_bearer_token(headers: &HeaderMap<HeaderValue>) -> Option<String> {
    headers
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| {
            if value.starts_with("Bearer ") {
                Some(value[7..].to_string())
            } else {
                Some(value.to_string())
            }
        })
}
