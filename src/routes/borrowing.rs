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
        borrowing::{Borrowing, BorrowingForUpdate},
        Engine,
    },
    state::AppState,
};

#[derive(Deserialize)]
struct PathParam {
    borrowing_id: i64,
}

async fn get_borrowing(
    State(state): State<AppState<Engine>>,
    Path(param): Path<PathParam>,
) -> Response {
    match Borrowing::get(&state, param.borrowing_id).await {
        Ok(borrowing) => (StatusCode::OK, Json(json!({ "borrowing": borrowing }))).into_response(),
        Err(e) => {
            error!("{e}");
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Borrowing not found" })),
            )
                .into_response()
        }
    }
}

async fn get_current_user_borrowings(
    Claims { user_id, .. }: Claims,
    State(state): State<AppState<Engine>>,
) -> Response {
    match Borrowing::list_by_user(&state, user_id).await {
        Ok(borrowings) => {
            (StatusCode::OK, Json(json!({ "borrowings": borrowings }))).into_response()
        }
        Err(e) => {
            error!("{e}");
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Borrowing not found" })),
            )
                .into_response()
        }
    }
}

async fn update_borrowing(
    State(state): State<AppState<Engine>>,
    Path(param): Path<PathParam>,
    Json(borrowing): Json<BorrowingForUpdate>,
) -> Response {
    match Borrowing::update(&state, param.borrowing_id, borrowing).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "message": "Borrowing updated" })),
        )
            .into_response(),
        Err(e) => {
            error!("{e}");
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Borrowing not found" })),
            )
                .into_response()
        }
    }
}

pub fn routes() -> Router<AppState<Engine>> {
    let admin_routes = Router::new()
        .route("/borrowing/{borrowing_id}", put(update_borrowing))
        .route_layer(middleware::from_fn(require_admin_role));

    let restricted = Router::new()
        .route("/borrowing/{borrowing_id}", get(get_borrowing))
        .merge(admin_routes)
        .route_layer(middleware::from_fn(require_issuer_admin_role));

    Router::new()
        .merge(restricted)
        .route("/borrowings", get(get_current_user_borrowings))
}
