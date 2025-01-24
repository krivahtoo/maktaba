use axum::{
    body::Body, extract::Request, http::StatusCode, middleware::Next, response::Response, Json,
};
use serde_json::{json, Value};
use tracing::debug;

use crate::{auth::Claims, model::user::UserRole};

// Middleware for filtering user roles
pub async fn require_role(
    role: UserRole,
    claims: Claims,
    req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, Json<Value>)> {
    debug!("role");
    // Check if claims exist in the request extensions
    if claims.role == role {
        Ok(next.run(req).await)
    } else {
        Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Unauthorized" })),
        ))
    }
}
