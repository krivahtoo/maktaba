use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post, put},
    Router,
};
use serde::Deserialize;
use serde_json::json;
use tracing::error;

use crate::{
    auth::Claims,
    extractors::{json::Json, path::Path},
    middlewares::role::require_admin_role,
    model::{
        book::{
            Book, BookCopyForCreate, BookCopyForUpdate, BookForCreate, BookForUpdate, BorrowStatus,
        },
        borrowing::{Borrowing, BorrowingForCreate},
        Engine,
    },
    state::AppState,
};

#[derive(Deserialize)]
struct PathParam {
    book_id: i64,
    #[serde(default)]
    copy_id: i64,
}

async fn add_book(
    State(state): State<AppState<Engine>>,
    Json(book): Json<BookForCreate>,
) -> Response {
    match Book::create(&state, book).await {
        Ok(id) => (StatusCode::CREATED, Json(json!({ "book_id": id }))).into_response(),
        Err(e) => {
            error!("{e}");
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Book not created" })),
            )
                .into_response()
        }
    }
}

async fn add_book_copy(
    State(state): State<AppState<Engine>>,
    Path(param): Path<PathParam>,
    Json(mut book): Json<BookCopyForCreate>,
) -> Response {
    book.book_id = param.book_id;
    match Book::add_copy(&state, book).await {
        Ok(id) => (StatusCode::CREATED, Json(json!({ "book_id": id }))).into_response(),
        Err(e) => {
            error!("{e}");
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Book not created" })),
            )
                .into_response()
        }
    }
}

async fn get_book(State(state): State<AppState<Engine>>, Path(param): Path<PathParam>) -> Response {
    match Book::get(&state, param.book_id).await {
        Ok(book) => (StatusCode::OK, Json(json!({ "book": book }))).into_response(),
        Err(e) => {
            error!("{e}");
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Book not found" })),
            )
                .into_response()
        }
    }
}

async fn get_book_copy(
    State(state): State<AppState<Engine>>,
    Path(param): Path<PathParam>,
) -> Response {
    match Book::get_copy(&state, param.copy_id, param.book_id).await {
        Ok(copy) => (StatusCode::OK, Json(json!({ "copy": copy }))).into_response(),
        Err(e) => {
            error!("{e}");
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Book copy not found" })),
            )
                .into_response()
        }
    }
}

async fn get_books(State(state): State<AppState<Engine>>) -> Response {
    match Book::list(&state).await {
        Ok(books) => (StatusCode::OK, Json(json!({ "books": books }))).into_response(),
        Err(e) => {
            error!("{e}");
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Book not found" })),
            )
                .into_response()
        }
    }
}

async fn get_book_copies(
    State(state): State<AppState<Engine>>,
    Path(param): Path<PathParam>,
) -> Response {
    match Book::list_copies(&state, param.book_id).await {
        Ok(copies) => (StatusCode::OK, Json(json!({ "copies": copies }))).into_response(),
        Err(e) => {
            error!("{e}");
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Book not found" })),
            )
                .into_response()
        }
    }
}

async fn update_book(
    State(state): State<AppState<Engine>>,
    Path(param): Path<PathParam>,
    Json(book): Json<BookForUpdate>,
) -> Response {
    match Book::update(&state, param.book_id, book).await {
        Ok(id) => (StatusCode::CREATED, Json(json!({ "book_id": id }))).into_response(),
        Err(e) => {
            error!("{e}");
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Book not created" })),
            )
                .into_response()
        }
    }
}

async fn update_book_copy(
    State(state): State<AppState<Engine>>,
    Path(PathParam { book_id, copy_id }): Path<PathParam>,
    Json(book): Json<BookCopyForUpdate>,
) -> Response {
    match Book::update_copy(&state, copy_id, book_id, book).await {
        Ok(id) => (StatusCode::CREATED, Json(json!({ "book_id": id }))).into_response(),
        Err(e) => {
            error!("{e}");
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Book not created" })),
            )
                .into_response()
        }
    }
}

async fn borrow_book_copy(
    State(state): State<AppState<Engine>>,
    Claims { user_id, .. }: Claims,
    Path(PathParam { book_id, copy_id }): Path<PathParam>,
) -> Response {
    match Book::get_copy(&state, copy_id, book_id).await {
        Ok(copy) => match copy.status {
            Some(BorrowStatus::Available) => match Borrowing::create(
                &state,
                BorrowingForCreate {
                    user_id,
                    book_id,
                    copy_id,
                },
            )
            .await
            {
                Ok(_) => {
                    (StatusCode::OK, Json(json!({ "message": "Book borrowed" }))).into_response()
                }
                Err(e) => {
                    error!("{e}");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({ "error": "Something is not right" })),
                    )
                        .into_response()
                }
            },
            Some(_) => (
                StatusCode::MISDIRECTED_REQUEST,
                Json(json!({ "error": "Book copy is not available" })),
            )
                .into_response(),
            None => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Something is not right" })),
            )
                .into_response(),
        },
        Err(e) => {
            error!("{e}");
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Book copy not found" })),
            )
                .into_response()
        }
    }
}

pub fn routes() -> Router<AppState<Engine>> {
    let admin_routes = Router::new()
        .route("/book", post(add_book))
        .route("/book/{book_id}", put(update_book))
        .route("/book/{book_id}/copy", post(add_book_copy))
        .route("/book/{book_id}/copy/{copy_id}", put(update_book_copy))
        .route_layer(axum::middleware::from_fn(require_admin_role));

    Router::new()
        .merge(admin_routes)
        .route("/book", get(get_books))
        .route("/books", get(get_books))
        .route("/book/{book_id}", get(get_book))
        .route("/book/{book_id}/copy", get(get_book_copies))
        .route("/book/{book_id}/copies", get(get_book_copies))
        .route("/book/{book_id}/copy/{copy_id}", get(get_book_copy))
        .route(
            "/book/{book_id}/copy/{copy_id}/borrow",
            get(borrow_book_copy),
        )
}
