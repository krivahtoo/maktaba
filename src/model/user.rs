use chrono::NaiveDateTime;
use modql::field::{Fields, HasSeaFields, SeaFieldValue};
use sea_query::{Expr, Iden, Query, SimpleExpr, SqliteQueryBuilder};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{query_as_with, FromRow, Type};
use uuid::Uuid;

use crate::{auth::hash, state::AppState};

use super::{Model, Result};

#[derive(Debug, Serialize, FromRow, Fields)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub role: UserRole,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub email: String,
    pub phone: Option<String>,
    pub photo: Option<String>,
    pub address: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize, Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
pub enum UserRole {
    #[default]
    Member,
    Issuer,
    Admin,
}

impl From<UserRole> for sea_query::Value {
    fn from(val: UserRole) -> Self {
        match val {
            UserRole::Member => "member".into(),
            UserRole::Issuer => "issuer".into(),
            UserRole::Admin => "admin".into(),
        }
    }
}

impl sea_query::Nullable for UserRole {
    fn null() -> sea_query::Value {
        sea_query::Value::String(None)
    }
}

#[derive(Debug, Deserialize, Fields)]
pub struct UserForCreate {
    pub name: String,
    #[serde(default)]
    pub role: UserRole,
    pub username: String,
    pub password: String,
    pub email: String,
    pub phone: Option<String>,
    pub photo: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Deserialize, Fields)]
pub struct UserForUpdate {
    pub name: Option<String>,
    #[serde(skip_deserializing)]
    pub role: Option<UserRole>,
    pub username: Option<String>,
    #[serde(skip_deserializing)]
    pub password: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub photo: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Deserialize, FromRow, Fields)]
pub struct UserForLogin {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct PasswordUpdate {
    pub old: String,
    pub new: String,
}
/// Marker trait
pub trait UserBy: HasSeaFields + for<'r> FromRow<'r, super::Row> + Unpin + Send {}

impl UserBy for User {}
impl UserBy for UserForLogin {}

#[derive(Iden)]
enum UserIden {
    Id,
    Username,
    Password,
}

impl Model for User {
    const TABLE: &'static str = "Users";
}

impl User {
    pub async fn get<E>(state: &AppState<super::Engine>, id: i64) -> Result<E>
    where
        E: UserBy,
    {
        super::get::<Self, _>(state, id).await
    }

    pub async fn get_by_username<E>(
        state: &AppState<super::Engine>,
        username: String,
    ) -> Result<Option<E>>
    where
        E: UserBy,
    {
        let db = &state.pool;

        let mut query = Query::select();
        query
            .from(Self::table_ref())
            .columns(E::sea_idens())
            .and_where(Expr::col(UserIden::Username).eq(username));

        let (sql, values) = query.build_sqlx(SqliteQueryBuilder);
        let user = query_as_with::<_, E, _>(&sql, values)
            .fetch_optional(db)
            .await?;

        Ok(user)
    }

    pub async fn create(state: &AppState<super::Engine>, mut user: UserForCreate) -> Result<i64> {
        let password =
            hash(&user.password).map_err(|e| super::error::Error::Hash(e.to_string()))?;
        super::create::<Self, _>(state, UserForCreate { password, ..user }).await
    }

    pub async fn update(
        state: &AppState<super::Engine>,
        id: i64,
        mut user: UserForUpdate,
    ) -> Result<()> {
        if let Some(password) = user.password {
            user.password =
                Some(hash(&password).map_err(|e| super::error::Error::Hash(e.to_string()))?);
        };
        super::update::<Self, _>(state, id, user).await
    }

    pub async fn list<E>(state: &AppState<super::Engine>) -> Result<Vec<E>>
    where
        E: UserBy,
    {
        super::list::<Self, _>(state).await
    }

    pub async fn count(state: &AppState<super::Engine>) -> Result<i64> {
        let db = &state.pool;

        let mut query = Query::select();
        query
            .expr(Expr::expr(Expr::value("*")).count())
            .from(Self::table_ref());

        let (sql, values) = query.build_sqlx(SqliteQueryBuilder);
        let (count,) = query_as_with::<_, (i64,), _>(&sql, values)
            .fetch_one(db)
            .await?;

        Ok(count)
    }

    pub async fn delete(state: &AppState<super::Engine>, id: i64) -> Result<()> {
        super::delete::<Self>(state, id).await
    }
}
