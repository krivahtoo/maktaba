use std::sync::Arc;

use axum::{middleware, Router};
use listenfd::ListenFd;
use sqlx::SqlitePool;
use state::AppStateInner;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tracing::{debug, info};

use self::{error::Result, middlewares::log::request_logger};

mod assets;
mod auth;
mod error;
mod extractors;
mod middlewares;
mod model;
mod routes;
mod state;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let db_url = std::env::var("DATABASE_URL").unwrap_or("sqlite://:memory:".to_string());
    let pool = SqlitePool::connect(&db_url).await?;
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or("secret".to_string());

    sqlx::migrate!().run(&pool).await?;

    // build our application with a route
    let app = Router::new()
        .merge(routes::routes(Arc::new(AppStateInner { pool, jwt_secret })))
        // handle all other routes from the frontend
        .fallback(assets::static_handler)
        .layer(middleware::from_fn(request_logger))
        .layer(CookieManagerLayer::new())
        ;

    // run our app with hyper, listening globally on port 3000
    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0)? {
        // if we are given a tcp listener on listen fd 0, we use that one
        Some(listener) => {
            info!("Running in dev auto reload");
            listener.set_nonblocking(true)?;
            TcpListener::from_std(listener)?
        }
        // otherwise fall back to local listening
        None => TcpListener::bind("0.0.0.0:3000").await?,
    };
    info!("Listening on port 3000");
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            utils::shutdown_signal().await;
            info!("Ctrl+C Received, Shutting down");
        })
        .await?;
    debug!("Bye");
    Ok(())
}
