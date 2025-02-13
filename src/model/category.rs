use chrono::{NaiveDate, NaiveDateTime};
use modql::field::Fields;
use serde::Serialize;
use sqlx::prelude::FromRow;

use super::Model;

#[derive(Debug, Serialize, FromRow, Fields)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub updated_at: Option<NaiveDateTime>,
}

impl Model for Category {
    const TABLE: &'static str = "Categories";
}
