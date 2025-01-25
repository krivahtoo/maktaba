use axum::{
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde_json::json;

use crate::{middlewares::require_login, model::Engine, state::AppState};

mod auth;
mod user;

// basic handler that responds with a hello world json
async fn hello_world() -> Response {
    (StatusCode::OK, Json(json!({ "hello": "Hello, World!" }))).into_response()
}

pub fn routes(state: AppState<Engine>) -> Router {
    let protected_routes = Router::new()
        .route("/hello", get(hello_world))
        .merge(user::routes())
        .route_layer(middleware::from_fn(require_login));

    let api_routes = Router::new()
        .merge(protected_routes)
        .merge(auth::routes())
        .fallback(not_found);

    Router::new()
        .route("/hello", get(hello_world))
        .nest("/api", api_routes)
        .with_state(state)
}

// Handle unknown '/api/xxx' endpoints
async fn not_found() -> Response {
    (StatusCode::NOT_FOUND, Json(json!({ "error": "Not found" }))).into_response()
}
