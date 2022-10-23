use super::db_query;
use crate::{
    cores::{
        database::pg::{db_time_now, DbPool},
        error::service::Error,
    },
    services::order::model::{entity::Order, request::OrderRequest},
};
use async_trait::async_trait;
use sqlx::postgres::PgRow;
use sqlx::Row;
use std::sync::Arc;
use uuid::Uuid;

struct DbRepository {
    pool: Arc<DbPool>,
}

pub fn new(pool: Arc<DbPool>) -> Arc<dyn DbRepo> {
    Arc::new(DbRepository { pool })
}

#[async_trait]
pub trait DbRepo: Sync + Send {
    async fn insert(&self, order: &OrderRequest, actor: &Uuid) -> Result<(), Error>;
    async fn get(&self, id: &Uuid) -> Result<Order, Error>;
}

#[async_trait]
impl DbRepo for DbRepository {
    async fn insert(&self, order: &OrderRequest, actor: &Uuid) -> Result<(), Error> {
        tracing::info!("Database Execute - Order Creation Query");

        let time_now = db_time_now();
        let id = Uuid::new_v4();
        let order_id = match &order.order_id {
            Some(order_id) => order_id.clone(),
            None => id.to_string(),
        };

        sqlx::query(db_query::ORDER_INSERT)
            .bind(&id)
            .bind(&order_id)
            .bind(&order.business)
            .bind(&order.state)
            .bind(time_now)
            .bind(actor)
            .bind(time_now)
            .bind(actor)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    async fn get(&self, id: &Uuid) -> Result<Order, Error> {
        tracing::info!("Database Execute - Order Get Query");

        let res = sqlx::query(db_query::ORDER_GET)
            .bind(id)
            .map(|row: PgRow| Order {
                id: row.get("id"),
                order_id: row.get("order_id"),
                business: row.get("business"),
                state: row.get("state"),
                create_time: row.get("create_time"),
                create_by: row.get("create_by"),
                update_time: row.get("update_time"),
                update_by: row.get("update_by"),
            })
            .fetch_one(self.pool.as_ref())
            .await?;
        Ok(res)
    }
}
