use axum::{
    extract::State,
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde_json::json;
use tracing::error;

use crate::{
    middlewares::require_login,
    model::{user::User, Engine},
    state::AppState,
};

mod auth;
mod book;
mod borrowing;
mod category;
mod fine;
mod reservation;
mod review;
mod user;

// basic handler that responds with a hello world json
async fn hello_world() -> Response {
    (StatusCode::OK, Json(json!({ "hello": "Hello, World!" }))).into_response()
}

async fn user_exists(State(state): State<AppState<Engine>>) -> Response {
    match User::count(&state).await {
        Ok(count) => {
            if count > 0 {
                (
                    StatusCode::OK,
                    Json(json!({
                        "status": true,
                        "message": "Users exists",
                        "count": count
                    })),
                )
                    .into_response()
            } else {
                (
                    StatusCode::OK,
                    Json(json!({ "status": false, "message": "No user" })),
                )
                    .into_response()
            }
        }
        Err(e) => {
            error!("{e}");
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "User not found" })),
            )
                .into_response()
        }
    }
}

pub fn routes(state: AppState<Engine>) -> Router {
    let protected_routes = Router::new()
        .route("/hello", get(hello_world))
        .merge(user::routes())
        .merge(book::routes())
        .merge(borrowing::routes())
        .merge(category::routes())
        .merge(fine::routes())
        .merge(review::routes())
        .merge(reservation::routes())
        .route_layer(middleware::from_fn(require_login));

    let api_routes = Router::new()
        .merge(protected_routes)
        .merge(auth::routes())
        .route("/users/exists", get(user_exists))
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
