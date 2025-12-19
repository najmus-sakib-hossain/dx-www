// Authentication middleware for dx-server

use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

/// Login request
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Login response with token
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: Vec<u8>, // 64-byte binary token
    pub expires_at: i64,
}

/// Handle login
#[cfg(feature = "auth")]
pub async fn handle_login(
    State(_state): State<crate::ServerState>,
    Json(req): Json<LoginRequest>,
) -> impl IntoResponse {
    // Verify credentials (placeholder)
    if !verify_credentials(&req.email, &req.password).await {
        return (StatusCode::UNAUTHORIZED, Json(None));
    }

    // Token generation disabled for now - needs ecosystem integration
    // In production, would use a TokenGenerator from server state
    (
        StatusCode::OK,
        Json(Some(LoginResponse {
            token: vec![0u8; 64], // Placeholder token
            expires_at: 0,
        })),
    )
}

/// Verify token middleware
#[cfg(feature = "auth")]
pub async fn verify_token_middleware(
    State(_state): State<crate::ServerState>,
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract token from header
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Verify token (placeholder)
    // In production, would verify signature
    if token.len() != 64 {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Add user info to request extensions
    // req.extensions_mut().insert(user_id);

    Ok(next.run(req).await)
}

/// Verify credentials (placeholder)
async fn verify_credentials(email: &str, password: &str) -> bool {
    // In production, would hash password and check DB
    email.contains('@') && password.len() >= 8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_verify_credentials() {
        assert!(verify_credentials("test@example.com", "password123").await);
        assert!(!verify_credentials("test", "short").await);
    }
}
