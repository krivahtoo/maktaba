use std::time::Instant;

use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use tracing::debug;

/// Middleware to log the response status and duration
pub async fn request_logger(req: Request<Body>, next: Next) -> Response {
    let start_time = Instant::now();

    let method = req.method().clone();
    let uri = req.uri().clone();

    // Proceed to the next middleware or handler
    let response = next.run(req).await;

    let duration = start_time.elapsed();

    debug!(
        "{} '{}' -> {} (took {:.2?})",
        method,
        uri,
        response.status(),
        duration
    );

    response
}
