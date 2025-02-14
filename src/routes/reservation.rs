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
        reservation::{Reservation, ReservationForCreate, ReservationForUpdate},
        Engine,
    },
    state::AppState,
};

#[derive(Deserialize)]
struct PathParam {
    reservation_id: i64,
}

async fn get_reservation(
    State(state): State<AppState<Engine>>,
    Path(param): Path<PathParam>,
) -> Response {
    match Reservation::get(&state, param.reservation_id).await {
        Ok(reservation) => (StatusCode::OK, Json(json!({ "reservation": reservation }))).into_response(),
        Err(e) => {
            error!("{e}");
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Reservation not found" })),
            )
                .into_response()
        }
    }
}

async fn update_reservation(
    State(state): State<AppState<Engine>>,
    Path(param): Path<PathParam>,
    Json(reservation): Json<ReservationForUpdate>,
) -> Response {
    match Reservation::update(&state, param.reservation_id, reservation).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "message": "Reservation updated" })),
        )
            .into_response(),
        Err(e) => {
            error!("{e}");
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Reservation not found" })),
            )
                .into_response()
        }
    }
}

async fn create_reservation(
    State(state): State<AppState<Engine>>,
    Json(reservation): Json<ReservationForCreate>,
) -> Response {
    match Reservation::create(&state, reservation).await {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({ "message": "Reservation added" })),
        )
            .into_response(),
        Err(e) => {
            error!("{e}");
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Reservation could not be added" })),
            )
                .into_response()
        }
    }
}

async fn get_reservations(State(state): State<AppState<Engine>>) -> Response {
    match Reservation::list(&state).await {
        Ok(reservations) => {
            (StatusCode::OK, Json(json!({ "reservations": reservations }))).into_response()
        }
        Err(e) => {
            error!("{e}");
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Reservation not found" })),
            )
                .into_response()
        }
    }
}

pub fn routes() -> Router<AppState<Engine>> {
    let admin_routes = Router::new()
        .route("/reservation/{reservation_id}", put(update_reservation))
        .route_layer(middleware::from_fn(require_admin_role));

    let restricted = Router::new()
        .route("/reservation", post(create_reservation))
        .merge(admin_routes)
        .route_layer(middleware::from_fn(require_issuer_admin_role));

    Router::new()
        .merge(restricted)
        .route("/reservations", get(get_reservations))
        .route("/reservation/{reservation_id}", get(get_reservation))
}
