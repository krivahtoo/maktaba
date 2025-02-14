use axum::{
    extract::State,
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::{get, post, put},
    Router,
};
use serde::Deserialize;
use serde_json::json;
use tracing::error;

use crate::{
    extractors::{json::Json, path::Path},
    middlewares::role::{require_admin_role, require_issuer_admin_role},
    model::{
        review::{Review, ReviewForCreate, ReviewForUpdate},
        Engine,
    },
    state::AppState,
};

#[derive(Deserialize)]
struct PathParam {
    review_id: i64,
}

async fn get_review(
    State(state): State<AppState<Engine>>,
    Path(param): Path<PathParam>,
) -> Response {
    match Review::get(&state, param.review_id).await {
        Ok(review) => (StatusCode::OK, Json(json!({ "review": review }))).into_response(),
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

async fn update_review(
    State(state): State<AppState<Engine>>,
    Path(param): Path<PathParam>,
    Json(review): Json<ReviewForUpdate>,
) -> Response {
    match Review::update(&state, param.review_id, review).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "message": "Review updated" })),
        )
            .into_response(),
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

async fn create_review(
    State(state): State<AppState<Engine>>,
    Json(review): Json<ReviewForCreate>,
) -> Response {
    match Review::create(&state, review).await {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({ "message": "Review added" })),
        )
            .into_response(),
        Err(e) => {
            error!("{e}");
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Review could not be added" })),
            )
                .into_response()
        }
    }
}

async fn get_reviews(State(state): State<AppState<Engine>>) -> Response {
    match Review::list(&state).await {
        Ok(reviews) => {
            (StatusCode::OK, Json(json!({ "reviews": reviews }))).into_response()
        }
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
        .route("/review/{review_id}", put(update_review))
        .route_layer(middleware::from_fn(require_admin_role));

    let restricted = Router::new()
        .route("/review", post(create_review))
        .merge(admin_routes)
        .route_layer(middleware::from_fn(require_issuer_admin_role));

    Router::new()
        .merge(restricted)
        .route("/reviews", get(get_reviews))
        .route("/review/{review_id}", get(get_review))
}
