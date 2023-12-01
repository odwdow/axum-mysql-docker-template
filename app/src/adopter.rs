mod transaction;
mod user;

use std::sync::Arc;

use sqlx::MySqlPool;
use tokio::sync::Mutex;
use crate::context::db::ArcPool;
use crate::context::adopter::{DBAdopter, Tx};
use transaction::{repository::TransactionRepository, query::TransactionQuery};
use user::{repository::UserRepository, query::UserQuery};

pub struct Adopter {
    pub pool: ArcPool,
    pub transaction: DBAdopter<TransactionRepository, TransactionQuery>,
    pub user: DBAdopter<UserRepository, UserQuery>,
}

impl Adopter {
    pub async fn new(pool: MySqlPool) -> Self {
        let pool = ArcPool::new(pool).await;
        let tx = Arc::new(Mutex::new(pool.clone().0.begin().await.unwrap()));

        Self {
            pool: pool.clone(),
            transaction: DBAdopter::new(pool.clone()),
            user: DBAdopter::new(pool.clone()),
        }
    }

    pub async fn start_transaction(&self) -> Tx {
        self.pool.0.begin().await.unwrap()
    }
}
