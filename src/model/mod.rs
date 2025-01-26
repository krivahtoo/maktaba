#![allow(unused)] // TODO: remove


use modql::{field::HasSeaFields, SIden};
use sea_query::{Expr, Iden, IntoIden, Query, SqliteQueryBuilder, TableRef};
use sea_query_binder::SqlxBinder;
use sqlx::{query_as_with, query_with, Database, FromRow};

use crate::state::AppState;

use error::Result;

pub mod book;
pub mod borrowing;
pub mod error;
pub mod user;

pub type Engine = sqlx::Sqlite;
type Row = sqlx::sqlite::SqliteRow;

#[derive(Iden)]
enum CommonIden {
    Id,
}

trait Model {
    const TABLE: &'static str;

    fn table_ref() -> TableRef {
        TableRef::Table(SIden(Self::TABLE).into_iden())
    }
}

async fn create<M, E>(state: &AppState<Engine>, data: E) -> Result<i64>
where
    M: Model,
    E: HasSeaFields,
{
    let db = &state.pool;

    let fields = data.not_none_sea_fields();
    let (columns, sea_values) = fields.for_sea_insert();

    let mut query = Query::insert();
    query
        .into_table(M::table_ref())
        .columns(columns)
        .values(sea_values)?
        .returning_col(CommonIden::Id);

    let (sql, values) = query.build_sqlx(SqliteQueryBuilder);
    let (id,) = query_as_with::<_, (i64,), _>(&sql, values)
        .fetch_one(db)
        .await?;
    Ok(id)
}

async fn get<M, E>(state: &AppState<Engine>, id: i64) -> Result<E>
where
    M: Model,
    E: for<'r> FromRow<'r, Row> + Unpin + Send,
    E: HasSeaFields,
{
    let db = &state.pool;

    let mut query = Query::select();
    query
        .from(M::table_ref())
        .columns(E::sea_idens())
        .and_where(Expr::col(CommonIden::Id).eq(id));

    let (sql, values) = query.build_sqlx(SqliteQueryBuilder);
    let entity = query_as_with::<_, E, _>(&sql, values)
        .fetch_optional(db)
        .await?
        .ok_or(error::Error::EntityNotFound {
            entity: M::TABLE,
            id,
        })?;

    Ok(entity)
}

async fn list<M, E>(state: &AppState<Engine>) -> Result<Vec<E>>
where
    M: Model,
    E: for<'r> FromRow<'r, Row> + Unpin + Send,
    E: HasSeaFields,
{
    let db = &state.pool;

    let mut query = Query::select();
    query.from(M::table_ref()).columns(E::sea_idens());

    let (sql, values) = query.build_sqlx(SqliteQueryBuilder);
    let entities = query_as_with::<_, E, _>(&sql, values).fetch_all(db).await?;

    Ok(entities)
}

async fn list_where<M, E, C, V>(state: &AppState<Engine>, col: C, val: V) -> Result<Vec<E>>
where
    M: Model,
    E: for<'r> FromRow<'r, Row> + Unpin + Send,
    E: HasSeaFields,
    C: IntoColumnRef,
    V: Into<SimpleExpr>,
{
    let db = &state.pool;

    let mut query = Query::select();
    query
        .from(M::table_ref())
        .columns(E::sea_idens())
        .and_where(Expr::col(col).eq(val));

    let (sql, values) = query.build_sqlx(SqliteQueryBuilder);
    let entities = query_as_with::<_, E, _>(&sql, values).fetch_all(db).await?;

    Ok(entities)
}

async fn update<M, E>(state: &AppState<Engine>, id: i64, data: E) -> Result<()>
where
    M: Model,
    E: HasSeaFields,
{
    let db = &state.pool;

    let fields = data.not_none_sea_fields();
    let fields = fields.for_sea_update();

    let mut query = Query::update();
    query
        .table(M::table_ref())
        .values(fields)
        .and_where(Expr::col(CommonIden::Id).eq(id));

    let (sql, values) = query.build_sqlx(SqliteQueryBuilder);
    match query_with(&sql, values).execute(db).await?.rows_affected() {
        0 => Err(error::Error::EntityNotFound {
            entity: M::TABLE,
            id,
        }),
        1 => Ok(()),
        _ => Err(error::Error::CountFail),
    }
}

async fn delete<M>(state: &AppState<Engine>, id: i64) -> Result<()>
where
    M: Model,
{
    let db = &state.pool;

    let mut query = Query::delete();
    query
        .from_table(M::table_ref())
        .and_where(Expr::col(CommonIden::Id).eq(id));

    let (sql, values) = query.build_sqlx(SqliteQueryBuilder);
    match query_with(&sql, values).execute(db).await?.rows_affected() {
        0 => Err(error::Error::EntityNotFound {
            entity: M::TABLE,
            id,
        }),
        1 => Ok(()),
        _ => Err(error::Error::CountFail),
    }
}
