use async_trait::async_trait;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;
use tokio::sync::RwLock;

#[async_trait]
pub trait Store: Sized + Sync + Send {
    type Pool;

    async fn new(db_url: &str) -> Result<Self, sqlx::Error>;
    fn connection(&self) -> &Self::Pool;
}

#[derive(Debug, Clone)]
pub struct PostgresStore {
    pub connection: PgPool,
}

#[async_trait]
impl Store for PostgresStore {
    type Pool = PgPool;

    async fn new(db_url: &str) -> Result<Self, sqlx::Error> {
        let db_pool = PgPoolOptions::new()
            .max_connections(30)
            .connect(db_url)
            .await?;

        // TODO May need to delete generic type
        Ok(PostgresStore {
            connection: db_pool,
        })
    }
    fn connection(&self) -> &Self::Pool {
        &self.connection
    }
}

pub(crate) type StoreType<T> = Arc<RwLock<T>>;
