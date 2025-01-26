use chrono::NaiveDate;
use modql::field::{Fields, HasSeaFields};
use sea_query::{Expr, Iden, Query, SqliteQueryBuilder};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{query_as_with, FromRow, Type};

use crate::state::AppState;

use super::{error::Result, Model};

#[derive(Debug, Serialize, FromRow, Fields)]
pub struct Borrowing {
    pub id: i64,
    pub user_id: i64,
    pub book_id: i64,
    pub copy_id: i64,
    pub borrow_date: NaiveDate,
    pub return_date: Option<NaiveDate>,
    pub status: BorrowingStatus,
}

#[derive(Debug, Default, Serialize, Deserialize, Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
pub enum BorrowingStatus {
    #[default]
    Borrowed,
    Returned,
    Late,
}

impl From<BorrowingStatus> for sea_query::Value {
    fn from(val: BorrowingStatus) -> Self {
        match val {
            BorrowingStatus::Borrowed => "borrowed".into(),
            BorrowingStatus::Returned => "returned".into(),
            BorrowingStatus::Late => "late".into(),
        }
    }
}

impl sea_query::Nullable for BorrowingStatus {
    fn null() -> sea_query::Value {
        sea_query::Value::String(None)
    }
}

#[derive(Debug, Deserialize, FromRow, Fields)]
pub struct BorrowingForCreate {
    pub user_id: i64,
    pub book_id: i64,
    pub copy_id: i64,
}

#[derive(Debug, Deserialize, FromRow, Fields)]
pub struct BorrowingForUpdate {
    pub return_date: Option<NaiveDate>,
    pub status: Option<BorrowingStatus>,
}

#[derive(Iden)]
enum BorrowingIden {
    UserId,
    CopyId,
    BookId,
    Status,
}

impl Model for Borrowing {
    const TABLE: &'static str = "Borrowing";
}

impl Borrowing {
    pub async fn get(state: &AppState<super::Engine>, id: i64) -> Result<Borrowing> {
        super::get::<Self, _>(state, id).await
    }

    pub async fn create(state: &AppState<super::Engine>, borrowing: BorrowingForCreate) -> Result<i64> {
        super::create::<Self, _>(state, borrowing).await
    }

    pub async fn list_by_user(
        state: &AppState<super::Engine>,
        user_id: i64,
    ) -> Result<Vec<Borrowing>> {
        super::list_where::<Self, _, _, _>(state, BorrowingIden::UserId, user_id).await
    }

    pub async fn list_by_book(
        state: &AppState<super::Engine>,
        book_id: i64,
    ) -> Result<Vec<Borrowing>> {
        super::list_where::<Self, _, _, _>(state, BorrowingIden::BookId, book_id).await
    }

    pub async fn list_by_status(
        state: &AppState<super::Engine>,
        status: BorrowingStatus,
    ) -> Result<Vec<Borrowing>> {
        super::list_where::<Self, _, _, _>(state, BorrowingIden::Status, status).await
    }

    pub async fn list_by_book_copy(
        state: &AppState<super::Engine>,
        book_id: i64,
        copy_id: i64,
    ) -> Result<Vec<Borrowing>> {
        let db = &state.pool;

        let mut query = Query::select();
        query
            .from(Self::table_ref())
            .columns(Self::sea_idens())
            .and_where(Expr::col(BorrowingIden::BookId).eq(book_id))
            .and_where(Expr::col(BorrowingIden::CopyId).eq(copy_id));

        let (sql, values) = query.build_sqlx(SqliteQueryBuilder);
        let entities = query_as_with::<_, Self, _>(&sql, values)
            .fetch_all(db)
            .await?;

        Ok(entities)
    }

    pub async fn update(
        state: &AppState<super::Engine>,
        id: i64,
        borrowing: BorrowingForUpdate,
    ) -> Result<()> {
        super::update::<Self, _>(state, id, borrowing).await
    }
}
