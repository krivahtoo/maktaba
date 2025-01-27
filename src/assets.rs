use axum::{
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Redirect, Response},
};
use rust_embed::Embed;
use tracing::debug;

use crate::auth::{AuthError, Claims};

static INDEX_HTML: &str = "index.html";
static FALLBACK_HTML: &str = "404.html";

#[derive(Embed)]
#[folder = "frontend/build/"]
struct Assets;

pub async fn static_handler(claims: Result<Claims, AuthError>, uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    if claims.is_err() && !path_public(path) {
        debug!("not authenticated redirecting");
        return Redirect::temporary("/login").into_response();
    }

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
            match Assets::get(&format!("{path}/{INDEX_HTML}")) {
                Some(content) => {
                    debug!("serving {path}/index.html");
                    ([(header::CONTENT_TYPE, "text/html")], content.data).into_response()
                }

                None => fallback().await,
            }
        }
    }
}

fn path_public(n: &str) -> bool {
    let public_paths = vec!["login", "register", "_app", "favicon.png", "robots.txt"];
    for path in public_paths {
        if n.starts_with(path) {
            return true;
        }
    }
    false
}

async fn index_html() -> Response {
    match Assets::get(INDEX_HTML) {
        Some(content) => Html(content.data).into_response(),
        None => not_found().await,
    }
}

async fn fallback() -> Response {
    match Assets::get(FALLBACK_HTML) {
        Some(content) => Html(content.data).into_response(),
        None => not_found().await,
    }
}

async fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "404").into_response()
}
