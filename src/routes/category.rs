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
        category::{Category, CategoryForUpdate},
        Engine,
    },
    state::AppState,
};

#[derive(Deserialize)]
struct PathParam {
    category_id: i64,
}

async fn get_category(
    State(state): State<AppState<Engine>>,
    Path(param): Path<PathParam>,
) -> Response {
    match Category::get(&state, param.category_id).await {
        Ok(category) => (StatusCode::OK, Json(json!({ "category": category }))).into_response(),
        Err(e) => {
            error!("{e}");
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Category not found" })),
            )
                .into_response()
        }
    }
}

async fn update_category(
    State(state): State<AppState<Engine>>,
    Path(param): Path<PathParam>,
    Json(category): Json<CategoryForUpdate>,
) -> Response {
    match Category::update(&state, param.category_id, category).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "message": "Category updated" })),
        )
            .into_response(),
        Err(e) => {
            error!("{e}");
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Category not found" })),
            )
                .into_response()
        }
    }
}

async fn create_category(
    State(state): State<AppState<Engine>>,
    Json(category): Json<CategoryForUpdate>,
) -> Response {
    match Category::create(&state, category).await {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({ "message": "Category added" })),
        )
            .into_response(),
        Err(e) => {
            error!("{e}");
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Category could not be added" })),
            )
                .into_response()
        }
    }
}

async fn get_categories(State(state): State<AppState<Engine>>) -> Response {
    match Category::list(&state).await {
        Ok(categories) => {
            (StatusCode::OK, Json(json!({ "categories": categories }))).into_response()
        }
        Err(e) => {
            error!("{e}");
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Category not found" })),
            )
                .into_response()
        }
    }
}

pub fn routes() -> Router<AppState<Engine>> {
    let admin_routes = Router::new()
        .route("/category/{category_id}", put(update_category))
        .route_layer(middleware::from_fn(require_admin_role));

    let restricted = Router::new()
        .route("/category", post(create_category))
        .merge(admin_routes)
        .route_layer(middleware::from_fn(require_issuer_admin_role));

    Router::new()
        .merge(restricted)
        .route("/categories", get(get_categories))
        .route("/category/{category_id}", get(get_category))
}
