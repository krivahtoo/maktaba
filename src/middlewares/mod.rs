use axum::{
    body::Body, extract::Request, http::StatusCode, middleware::Next, response::Response, Json,
};
use serde_json::{json, Value};

use crate::{auth::Claims, model::user::UserRole};

// Middleware for filtering user roles
pub async fn require_role(
    role: UserRole,
    req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, Json<Value>)> {
    // Check if claims exist in the request extensions
    if let Some(claims) = req.extensions().get::<Claims>() {
        if claims.role == role {
            return Ok(next.run(req).await);
        } else {
            return Err((StatusCode::FORBIDDEN, Json(json!({ "error": "Forbidden" }))));
        }
    }

    Err((
        StatusCode::UNAUTHORIZED,
        Json(json!({ "error": "Unauthorized" })),
    ))
}
