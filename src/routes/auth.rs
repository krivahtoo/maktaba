use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use chrono::Utc;
use serde_json::json;
use tower_cookies::{Cookie, Cookies};
use tracing::error;

use crate::{
    auth::{generate_jwt, verify, Claims},
    extractors::json::Json,
    model::{
        user::{User, UserForCreate, UserForLogin},
        Engine,
    },
    state::AppState,
};

async fn login(
    State(state): State<AppState<Engine>>,
    cookies: Cookies,
    Json(user): Json<UserForLogin>,
) -> Response {
    if let Some(u) = User::get_by_username::<User>(&state, user.username)
        .await
        .unwrap()
    {
        match verify(&u.password, &user.password) {
            Ok(_) => {
                let now = Utc::now().timestamp();
                let token = generate_jwt(
                    &Claims {
                        user_id: u.id,
                        role: u.role,
                        exp: now as usize + 3600,
                    },
                    &state.jwt_secret,
                )
                .unwrap();
                cookies.add(Cookie::new("token", token.clone()));
                (StatusCode::OK, Json(json!({ "token": token }))).into_response()
            }
            Err(_) => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Wrong username or password" })),
            )
                .into_response(),
        }
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Wrong username or password" })),
        )
            .into_response()
    }
}

async fn register(
    State(state): State<AppState<Engine>>,
    Json(user): Json<UserForCreate>,
) -> Response {
    match User::create(&state, user).await {
        Ok(id) => (
            StatusCode::CREATED,
            Json(json!({ "status": "Success", "user_id": id })),
        )
            .into_response(),
        Err(e) => {
            error!("{e}");
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Something doesn't look right" })),
            )
                .into_response()
        }
    }
}

async fn logout(cookies: Cookies) -> Response {
    cookies.remove("token".into());
    (StatusCode::OK, Json(json!({ "message": "Logout success" }))).into_response()
}

pub fn routes() -> Router<AppState<Engine>> {
    Router::new()
        .route("/logout", get(logout))
        .route("/login", post(login))
        .route("/register", post(register))
}
