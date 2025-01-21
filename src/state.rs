use std::sync::Arc;

use sqlx::{Database, Pool};

pub type AppState<T> = Arc<AppStateInner<T>>;

#[derive(Clone)]
pub struct AppStateInner<T: Database> {
    pub pool: Pool<T>,
    pub jwt_secret: String,
}
