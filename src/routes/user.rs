use axum::{
    extract::State,
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::{get, put},
    Router,
};
use serde::Deserialize;
use serde_json::json;
use tracing::error;

use crate::{
    extractors::{json::Json, path::Path},
    middlewares::role::{require_admin_role, require_issuer_admin_role},
    model::{
        user::{User, UserForUpdate},
        Engine,
    },
    state::AppState,
};

#[derive(Deserialize)]
struct PathParam {
    user_id: i64,
}

async fn get_user(State(state): State<AppState<Engine>>, Path(param): Path<PathParam>) -> Response {
    match User::get::<User>(&state, param.user_id).await {
        Ok(user) => (StatusCode::OK, Json(json!({ "user": user }))).into_response(),
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

async fn get_users(State(state): State<AppState<Engine>>) -> Response {
    match User::list::<User>(&state).await {
        Ok(users) => (StatusCode::OK, Json(json!({ "users": users }))).into_response(),
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

async fn update_user(
    State(state): State<AppState<Engine>>,
    Path(param): Path<PathParam>,
    Json(user): Json<UserForUpdate>,
) -> Response {
    match User::update(&state, param.user_id, user).await {
        Ok(users) => (StatusCode::OK, Json(json!({ "users": users }))).into_response(),
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

pub fn routes() -> Router<AppState<Engine>> {
    let admin_routes = Router::new()
        .route("/user/{user_id}", put(update_user))
        .route_layer(middleware::from_fn(require_admin_role));

    Router::new()
        .route("/user", get(get_users))
        .merge(admin_routes)
        .route("/user/{user_id}", get(get_user))
        .route_layer(middleware::from_fn(require_issuer_admin_role))
}
