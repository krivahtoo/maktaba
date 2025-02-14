use chrono::{NaiveDate, NaiveDateTime};
use modql::field::Fields;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::state::AppState;

use super::{Model, Result};

#[derive(Debug, Serialize, FromRow, Fields)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Fields)]
pub struct CategoryForUpdate {
    pub name: String,
}

impl Model for Category {
    const TABLE: &'static str = "Categories";
}

impl Category {
    pub async fn get(state: &AppState<super::Engine>, id: i64) -> Result<Category> {
        super::get::<Self, _>(state, id).await
    }

    pub async fn update(
        state: &AppState<super::Engine>,
        id: i64,
        category: CategoryForUpdate,
    ) -> Result<()> {
        super::update::<Self, _>(state, id, category).await
    }

    pub async fn create(
        state: &AppState<super::Engine>,
        category: CategoryForUpdate,
    ) -> Result<i64> {
        super::create::<Self, _>(state, category).await
    }

    pub async fn list(state: &AppState<super::Engine>) -> Result<Vec<Category>> {
        super::list::<Self, _>(state).await
    }
}
