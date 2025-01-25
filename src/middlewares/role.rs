use axum::{body::Body, extract::Request, http::StatusCode, middleware::Next, response::Response};
use serde_json::{json, Value};
use tracing::debug;

use crate::{auth::Claims, extractors::json::Json, model::user::UserRole};

// Middleware for filtering admin users
pub async fn require_admin_role(
    claims: Claims,
    req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, Json<Value>)> {
    // Check if claims exist in the request extensions
    if let UserRole::Admin = claims.role {
        Ok(next.run(req).await)
    } else {
        debug!("User not admin");
        Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Unauthorized" })),
        ))
    }
}

// Middleware for filtering issuer and admin users
pub async fn require_issuer_admin_role(
    claims: Claims,
    req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, Json<Value>)> {
    // Check if claims exist in the request extensions
    if let UserRole::Admin | UserRole::Issuer = claims.role {
        Ok(next.run(req).await)
    } else {
        debug!("User not issuer or admin");
        Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Unauthorized" })),
        ))
    }
}
