use chrono::{NaiveDate, NaiveDateTime};
use modql::field::Fields;
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::state::AppState;

use super::{Model, Result};

#[derive(Debug, Serialize, FromRow, Fields)]
pub struct Review {
    pub id: i64,
    pub user_id: i64,
    pub book_id: i64,
    pub rating: i8,
    pub review_text: String,
    pub updated_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Fields)]
pub struct ReviewForCreate {
    pub user_id: i64,
    pub book_id: i64,
    pub rating: i8,
    pub review_text: String,
}

#[derive(Debug, Deserialize, Fields)]
pub struct ReviewForUpdate {
    pub user_id: Option<i64>,
    pub book_id: Option<i64>,
    pub rating: Option<i8>,
    pub review_text: Option<String>,
}

#[derive(Iden)]
enum ReviewIden {
    Id,
    BookId,
    Userid,
}

impl Model for Review {
    const TABLE: &'static str = "Reviews";
}

impl Review {
    pub async fn get(state: &AppState<super::Engine>, id: i64) -> Result<Review> {
        super::get::<Self, _>(state, id).await
    }

    pub async fn update(
        state: &AppState<super::Engine>,
        id: i64,
        review: ReviewForUpdate,
    ) -> Result<()> {
        super::update::<Self, _>(state, id, review).await
    }

    pub async fn create(state: &AppState<super::Engine>, review: ReviewForCreate) -> Result<i64> {
        super::create::<Self, _>(state, review).await
    }

    pub async fn list(state: &AppState<super::Engine>) -> Result<Vec<Review>> {
        super::list::<Self, _>(state).await
    }

    pub async fn list_by_book(
        state: &AppState<super::Engine>,
        book_id: i64,
    ) -> Result<Vec<Review>> {
        super::list_where::<Self, _, _, _>(state, ReviewIden::BookId, book_id).await
    }

    pub async fn list_by_user(
        state: &AppState<super::Engine>,
        user_id: i64,
    ) -> Result<Vec<Review>> {
        super::list_where::<Self, _, _, _>(state, ReviewIden::Userid, user_id).await
    }
}
