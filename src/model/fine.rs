use chrono::{NaiveDate, NaiveDateTime};
use modql::field::Fields;
use serde::Serialize;
use sqlx::prelude::FromRow;

use super::Model;

#[derive(Debug, Serialize, FromRow, Fields)]
pub struct Fine {
    pub id: u64,
    pub transaction_id: u64,
    pub fine_amount: f64,
    pub paid: bool,
    pub paid_date: Option<NaiveDate>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Model for Fine {
    const TABLE: &'static str = "Fines";
}
