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
        fine::{Fine, FineForUpdate},
        Engine,
    },
    state::AppState,
};

#[derive(Deserialize)]
struct PathParam {
    fine_id: i64,
}

async fn get_fine(
    State(state): State<AppState<Engine>>,
    Path(param): Path<PathParam>,
) -> Response {
    match Fine::get(&state, param.fine_id).await {
        Ok(fine) => (StatusCode::OK, Json(json!({ "fine": fine }))).into_response(),
        Err(e) => {
            error!("{e}");
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Fine not found" })),
            )
                .into_response()
        }
    }
}

async fn update_fine(
    State(state): State<AppState<Engine>>,
    Path(param): Path<PathParam>,
    Json(fine): Json<FineForUpdate>,
) -> Response {
    match Fine::update(&state, param.fine_id, fine).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "message": "Fine updated" })),
        )
            .into_response(),
        Err(e) => {
            error!("{e}");
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Fine not found" })),
            )
                .into_response()
        }
    }
}

async fn create_fine(
    State(state): State<AppState<Engine>>,
    Json(fine): Json<FineForUpdate>,
) -> Response {
    match Fine::create(&state, fine).await {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({ "message": "Fine added" })),
        )
            .into_response(),
        Err(e) => {
            error!("{e}");
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Fine could not be added" })),
            )
                .into_response()
        }
    }
}

async fn get_fines(State(state): State<AppState<Engine>>) -> Response {
    match Fine::list(&state).await {
        Ok(categories) => {
            (StatusCode::OK, Json(json!({ "categories": categories }))).into_response()
        }
        Err(e) => {
            error!("{e}");
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Fine not found" })),
            )
                .into_response()
        }
    }
}

pub fn routes() -> Router<AppState<Engine>> {
    let admin_routes = Router::new()
        .route("/fine/{fine_id}", put(update_fine))
        .route_layer(middleware::from_fn(require_admin_role));

    let restricted = Router::new()
        .route("/fine", post(create_fine))
        .merge(admin_routes)
        .route_layer(middleware::from_fn(require_issuer_admin_role));

    Router::new()
        .merge(restricted)
        .route("/fines", get(get_fines))
        .route("/fine/{fine_id}", get(get_fine))
}
