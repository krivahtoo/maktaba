use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use serde::Deserialize;
use serde_json::json;
use tracing::error;

use crate::{
    extractors::{json::Json, path::Path},
    model::{user::User, Engine},
    state::AppState,
};

#[derive(Deserialize)]
struct PathParam {
    user_id: i64,
}

async fn get_user(State(state): State<AppState<Engine>>, Path(param): Path<PathParam>) -> Response {
    match User::get::<User>(&state, param.user_id).await {
        Ok(u) => (StatusCode::OK, Json(json!({ "user": u }))).into_response(),
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

pub fn routes() -> Router<AppState<Engine>> {
    Router::new()
        .route("/user", get(get_users))
        .route("/user/{user_id}", get(get_user))
}
