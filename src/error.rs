use std::io;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    SqlxMigration(#[from] sqlx::migrate::MigrateError),
    #[error("{0}")]
    Argon2(String),
    #[error(transparent)]
    Jwt(#[from] jsonwebtoken::errors::Error),
}
