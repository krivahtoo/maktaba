use chrono::{NaiveDate, NaiveDateTime};
use modql::field::Fields;
use serde::Serialize;
use sqlx::prelude::FromRow;

use super::Model;

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


impl Model for Review {
    const TABLE: &'static str = "Reviews";
}
