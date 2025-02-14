use std::future::Pending;

use chrono::{NaiveDate, NaiveDateTime};
use modql::field::Fields;
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};

use crate::state::AppState;

use super::{Model, Result};

#[derive(Debug, Serialize, FromRow, Fields)]
pub struct Reservation {
    pub id: i64,
    pub copy_id: i64,
    pub book_id: i64,
    pub user_id: i64,
    pub reservation_date: NaiveDate,
    pub status: ReservationStatus,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
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

#[derive(Debug, Deserialize, Fields)]
pub struct ReservationForCreate {
    pub copy_id: i64,
    pub book_id: i64,
    pub user_id: i64,
    pub reservation_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize, Fields)]
pub struct ReservationForUpdate {
    pub status: ReservationStatus,
}

#[derive(Iden)]
enum ReservationIden {
    Id,
    BookId,
    Userid,
}

impl Model for Reservation {
    const TABLE: &'static str = "Reservations";
}

impl Reservation {
    pub async fn get(state: &AppState<super::Engine>, id: i64) -> Result<Reservation> {
        super::get::<Self, _>(state, id).await
    }

    pub async fn update(
        state: &AppState<super::Engine>,
        id: i64,
        review: ReservationForUpdate,
    ) -> Result<()> {
        super::update::<Self, _>(state, id, review).await
    }

    pub async fn create(
        state: &AppState<super::Engine>,
        review: ReservationForCreate,
    ) -> Result<i64> {
        super::create::<Self, _>(state, review).await
    }

    pub async fn list(state: &AppState<super::Engine>) -> Result<Vec<Reservation>> {
        super::list::<Self, _>(state).await
    }

    pub async fn list_by_user(
        state: &AppState<super::Engine>,
        user_id: i64,
    ) -> Result<Vec<Reservation>> {
        super::list_where::<Self, _, _, _>(state, ReservationIden::Userid, user_id).await
    }
}
