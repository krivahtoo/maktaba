use chrono::{NaiveDate, NaiveDateTime};
use modql::field::Fields;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::state::AppState;

use super::{Model, Result};

#[derive(Debug, Serialize, FromRow, Fields)]
pub struct Fine {
    pub id: u64,
    pub transaction_id: u64,
    pub fine_amount: f64,
    pub paid: bool,
    pub paid_date: Option<NaiveDate>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Fields)]
pub struct FineForCreate {
    pub transaction_id: u64,
    pub fine_amount: f64,
    pub paid: Option<bool>,
    pub paid_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize, Fields)]
pub struct FineForUpdate {
    pub fine_amount: Option<f64>,
    pub paid: Option<bool>,
    pub paid_date: Option<NaiveDate>,
}

impl Model for Fine {
    const TABLE: &'static str = "Fines";
}

impl Fine {
    pub async fn get(state: &AppState<super::Engine>, id: i64) -> Result<Fine> {
        super::get::<Self, _>(state, id).await
    }

    pub async fn update(
        state: &AppState<super::Engine>,
        id: i64,
        fine: FineForUpdate,
    ) -> Result<()> {
        super::update::<Self, _>(state, id, fine).await
    }

    pub async fn create(state: &AppState<super::Engine>, fine: FineForUpdate) -> Result<i64> {
        super::create::<Self, _>(state, fine).await
    }

    pub async fn list(state: &AppState<super::Engine>) -> Result<Vec<Fine>> {
        super::list::<Self, _>(state).await
    }
}
