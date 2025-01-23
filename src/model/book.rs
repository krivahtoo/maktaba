use chrono::NaiveDateTime;
use modql::field::{Fields, HasSeaFields, SeaFieldValue};
use sea_query::{Expr, Iden, Query, SqliteQueryBuilder};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{query_as_with, query_with, FromRow, Type};
use uuid::Uuid;

use crate::state::AppState;

use super::{Model, Result};

#[derive(Debug, Serialize, FromRow, Fields)]
pub struct Book {
    pub id: i64,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub category: Option<String>,
    pub year: Option<i32>,
    pub photo: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, FromRow, Fields)]
pub struct BookForCreate {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub category: Option<String>,
    pub year: Option<i32>,
    pub photo: Option<String>,
    #[field(skip)]
    #[sqlx(skip)]
    pub count: i32,
}

#[derive(Debug, Deserialize, FromRow, Fields)]
pub struct BookForUpdate {
    pub title: Option<String>,
    pub author: Option<String>,
    pub isbn: Option<String>,
    pub category: Option<String>,
    pub year: Option<i32>,
    pub photo: Option<String>,
    #[field(skip)]
    #[sqlx(skip)]
    pub count: i32,
}

#[derive(Debug, Serialize, FromRow, Fields)]
pub struct BookCopy {
    pub id: i64,
    pub book_id: i64,
    pub status: Option<BorrowStatus>,
    pub location: Option<String>,
    pub added_at: NaiveDateTime,
}

#[derive(Debug, Default, Deserialize, Serialize, Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
pub enum BorrowStatus {
    #[default]
    Available,
    Borrowed,
    Reserved,
}

impl From<BorrowStatus> for sea_query::Value {
    fn from(val: BorrowStatus) -> Self {
        match val {
            BorrowStatus::Available => "available".into(),
            BorrowStatus::Borrowed => "borrowed".into(),
            BorrowStatus::Reserved => "reserved".into(),
        }
    }
}

impl sea_query::Nullable for BorrowStatus {
    fn null() -> sea_query::Value {
        sea_query::Value::String(None)
    }
}

#[derive(Debug, Default, Serialize, FromRow, Fields)]
pub struct BookCopyForCreate {
    pub book_id: i64,
    pub status: Option<BorrowStatus>,
    pub location: Option<String>,
}

#[derive(Debug, Default, Deserialize, FromRow, Fields)]
pub struct BookCopyForUpdate {
    pub status: Option<BorrowStatus>,
    pub location: Option<String>,
}

#[derive(Iden)]
enum BookIden {
    Id,
    BookId,
    Status,
}

impl Model for Book {
    const TABLE: &'static str = "Books";
}

impl Model for BookCopy {
    const TABLE: &'static str = "BookCopies";
}

impl Book {
    pub async fn get(state: &AppState<super::Engine>, id: i64) -> Result<Book> {
        super::get::<Self, _>(state, id).await
    }

    pub async fn create(state: &AppState<super::Engine>, book: BookForCreate) -> Result<i64> {
        let count = book.count;
        let id = super::create::<Self, _>(state, book).await?;

        if count > 0 {
            for _ in (0..count) {
                super::create::<BookCopy, _>(
                    state,
                    BookCopyForCreate {
                        book_id: id,
                        ..Default::default()
                    },
                )
                .await?;
            }
        }

        Ok(id)
    }

    pub async fn add_copy(state: &AppState<super::Engine>, copy: BookCopyForCreate) -> Result<i64> {
        super::create::<Self, _>(state, copy).await
    }

    pub async fn update(
        state: &AppState<super::Engine>,
        id: i64,
        book: BookForUpdate,
    ) -> Result<()> {
        super::update::<Self, _>(state, id, book).await
    }

    pub async fn update_copy(
        state: &AppState<super::Engine>,
        copy_id: i64,
        book_id: i64,
        book: BookForUpdate,
    ) -> Result<()> {
        let db = &state.pool;

        let fields = book.not_none_sea_fields();
        let fields = fields.for_sea_update();

        let mut query = Query::update();
        query
            .table(BookCopy::table_ref())
            .values(fields)
            .and_where(Expr::col(BookIden::Id).eq(copy_id))
            .and_where(Expr::col(BookIden::BookId).eq(book_id));

        let (sql, values) = query.build_sqlx(SqliteQueryBuilder);
        match query_with(&sql, values).execute(db).await?.rows_affected() {
            0 => Err(super::error::Error::EntityNotFound {
                entity: BookCopy::TABLE,
                id: copy_id,
            }),
            1 => Ok(()),
            _ => Err(super::error::Error::CountFail),
        }
    }

    pub async fn delete(state: &AppState<super::Engine>, id: i64) -> Result<()> {
        super::delete::<Self>(state, id).await
    }
}
