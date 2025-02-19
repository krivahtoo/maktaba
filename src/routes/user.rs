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
    auth::Claims,
    extractors::{json::Json, path::Path},
    middlewares::role::{require_admin_role, require_issuer_admin_role},
    model::{
        review::Review,
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

async fn get_current_user(
    Claims { user_id, .. }: Claims,
    State(state): State<AppState<Engine>>,
) -> Response {
    match User::get::<User>(&state, user_id).await {
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
        Ok(_) => (StatusCode::OK, Json(json!({ "message": "User updated" }))).into_response(),
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

async fn update_current_user(
    State(state): State<AppState<Engine>>,
    Claims { user_id, .. }: Claims,
    Json(user): Json<UserForUpdate>,
) -> Response {
    match User::update(&state, user_id, user).await {
        Ok(_) => (StatusCode::OK, Json(json!({ "message": "User updated" }))).into_response(),
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

async fn get_reviews(
    State(state): State<AppState<Engine>>,
    Path(PathParam { user_id, .. }): Path<PathParam>,
) -> Response {
    match Review::list_by_user(&state, user_id).await {
        Ok(reviews) => (StatusCode::OK, Json(json!({ "reviews": reviews }))).into_response(),
        Err(e) => {
            error!("{e}");
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Review not found" })),
            )
                .into_response()
        }
    }
}

pub fn routes() -> Router<AppState<Engine>> {
    let admin_routes = Router::new()
        .route("/user/{user_id}", put(update_user))
        .route_layer(middleware::from_fn(require_admin_role));

    let restricted = Router::new()
        .route("/users", get(get_users))
        .route("/user/{user_id}", get(get_user))
        .merge(admin_routes)
        .route_layer(middleware::from_fn(require_issuer_admin_role));

    Router::new()
        .merge(restricted)
        .route("/user", get(get_current_user).put(update_current_user))
        .route("/user/reviews", get(get_reviews))
}
