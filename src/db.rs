use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use sqlx::postgres::{PgPool, PgPoolOptions};

#[async_trait]
pub trait Store<T: Sized + Send + Sync>: Sized {
    async fn new(db_url: &str) -> Result<Self, sqlx::Error>;
    fn connection(&self) -> &T;
}

#[derive(Debug, Clone)]
pub struct PostgresStore<T> {
    pub connection: T,
}

#[async_trait]
impl<T: Sized + Send + Sync> Store<T> for PostgresStore<T> {
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
    fn connection(&self) -> &P {
        &self.connection
    }
}

pub(crate) type StoreType<T> = Arc<RwLock<T>>;
