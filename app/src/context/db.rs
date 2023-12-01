use std::env;
use std::sync::Arc;
use anyhow::anyhow;
use sqlx::{Pool, MySql, mysql::MySqlPoolOptions, MySqlPool, Transaction};

#[derive(Clone)]
pub struct Db(pub(crate) Pool<MySql>);

impl Db {
    pub async fn new() -> Self {
        let url = format!(
            "mysql://root@{}:{}/codetest",
            Self::get_host(),
            Self::get_port()
        );

        tracing::debug!("Connecting to database. url: {}", url);

        let pool = MySqlPoolOptions::new()
            .max_connections(10)
            .connect(&url)
            .await
            .expect("can't connect to database");

        Self(pool)
    }

    fn get_host() -> String {
        env::var("DB_HOST").unwrap_or(String::from("127.0.0.1"))
    }

    fn get_port() -> String {
        env::var("DB_PORT").unwrap_or(String::from("3306"))
    }
}

pub struct ArcPool(pub(crate) Arc<MySqlPool>);

impl ArcPool {
    pub async fn new(pool: MySqlPool) -> Self {
        let pool = Arc::new(pool);

        Self(pool)
    }

    pub async fn start_transaction(&self) -> anyhow::Result<Transaction<'static, MySql>> {
        match self.0.begin().await {
            Ok(tr) => Ok(tr),
            Err(_) => Err(anyhow!("Can't start transaction."))
        }
    }

    pub fn clone(&self) -> Self {
        let pool = self.0.clone();

        Self(pool)
    }
}
