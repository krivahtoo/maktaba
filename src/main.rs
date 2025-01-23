use std::sync::Arc;

use axum::{
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    Router,
};
use rust_embed::Embed;
use sqlx::SqlitePool;
use state::AppStateInner;
use tokio::{net::TcpListener, signal};
use tower_cookies::CookieManagerLayer;
use tracing::{debug, info};

use self::error::Result;

mod error;
mod routes;
mod state;

static INDEX_HTML: &str = "index.html";

#[derive(Embed)]
#[folder = "frontend/build/"]
struct Assets;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let pool = SqlitePool::connect("sqlite://:memory:").await?;
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or("secret".to_string());

    sqlx::migrate!().run(&pool).await?;

    // build our application with a route
    let app = Router::new()
        .merge(routes::routes(Arc::new(AppStateInner { pool, jwt_secret })))
        // handle all other routes from the frontend
        .fallback(static_handler)
        .layer(CookieManagerLayer::new());

    // run our app with hyper, listening globally on port 3000
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    info!("Listening on port 3000");
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            shutdown_signal().await;
            info!("Ctrl+C Received, Shutting down");
        })
        .await?;
    debug!("Bye");
    Ok(())
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    debug!("Got path {path}");

    if path.is_empty() || path == INDEX_HTML {
        return index_html().await;
    }

    match Assets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => {
            if path.contains('.') {
                return not_found().await;
            }
            index_html().await
        }
    }
}

async fn index_html() -> Response {
    match Assets::get(INDEX_HTML) {
        Some(content) => Html(content.data).into_response(),
        None => not_found().await,
    }
}

async fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "404").into_response()
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
