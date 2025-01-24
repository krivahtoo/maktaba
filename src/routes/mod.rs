use axum::{
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde_json::json;

use crate::{
    middlewares::require_role,
    model::{user::UserRole, Engine},
    state::AppState,
};

mod auth;

// basic handler that responds with a static string
async fn hello_world() -> Response {
    (StatusCode::OK, Json(json!({ "hello": "Hello, World!" }))).into_response()
}

pub fn routes(state: AppState<Engine>) -> Router {
    let hello = Router::new()
        .route("/hello", get(hello_world))
        .route_layer(middleware::from_fn(|req, next| {
            require_role(UserRole::Admin, req, next)
        }));

    let api_routes = Router::new()
        .merge(hello)
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
