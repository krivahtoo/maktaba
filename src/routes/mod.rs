use axum::{routing::get, Router};
use sqlx::Database;

use crate::state::AppState;

// basic handler that responds with a static string
async fn hello_world() -> &'static str {
    "Hello, World!"
}

pub fn routes<T: Database>(state: AppState<T>) -> Router {
    let api_routes = Router::new()
        .route("/hello", get(hello_world));

    Router::new()
        .route("/hello", get(hello_world))
        .nest("/api", api_routes)
        .with_state(state)
}
