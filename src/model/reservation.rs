
use std::future::Pending;

use chrono::{NaiveDate, NaiveDateTime};
use modql::field::Fields;
use serde::Serialize;
use sqlx::prelude::{FromRow, Type};

use super::Model;

#[derive(Debug, Serialize, FromRow, Fields)]
pub struct Reservation {
    pub id: i64,
    pub copy_id: i64,
    pub book_id: i64,
    pub user_id: i64,
    pub reservation_date: NaiveDate,
    pub status: ReservationStatus,
}

#[derive(Debug, Serialize, Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
pub enum ReservationStatus {
    Pending,
    Active,
    Declined,
    Expired,
    Cancelled,
}

impl From<ReservationStatus> for sea_query::Value {
    fn from(val: ReservationStatus) -> Self {
        use ReservationStatus as RS;
        match val {
            RS::Pending => "pending".into(),
            RS::Active => "active".into(),
            RS::Expired => "expired".into(),
            RS::Cancelled => "cancelled".into(),
            RS::Declined => "declined".into(),
        }
    }
}

impl sea_query::Nullable for ReservationStatus {
    fn null() -> sea_query::Value {
        sea_query::Value::String(None)
    }
}
