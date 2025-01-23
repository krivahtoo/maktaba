pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    SeaQ(#[from] sea_query::error::Error),
    #[error("{id} not found in '{entity}'")]
    EntityNotFound {
        entity: &'static str,
        id: i64,
    },
    #[error("Count failure")]
    CountFail,
    #[error("Error hashing password {0}")]
    Hash(String),
}
