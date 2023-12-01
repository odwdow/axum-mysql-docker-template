use anyhow::anyhow;

use bigdecimal::ToPrimitive;
use sqlx::types::BigDecimal;
use crate::context::adopter::{Repository, Tx};
use crate::context::db::ArcPool;

pub struct UserRepository {
    pool: ArcPool,
}

impl Repository for UserRepository {
    fn new(pool: ArcPool) -> Self {
        Self {
            pool,
        }
    }
}

impl UserRepository {
    pub async fn get_total_transaction_for_update(
        &self, 
        tx: &mut Tx,
        user_id: &u32,
    ) -> anyhow::Result<u32> {
        tracing::debug!("TransactionRepository::get_total_transaction");

        let res = sqlx::query_scalar::<_, BigDecimal>(
            r"SELECT 
                  COALESCE(SUM(transactions.amount), 0)
              FROM users
              LEFT JOIN transactions ON users.id = transactions.user_id
              WHERE users.id = ?
              GROUP BY users.id
              FOR UPDATE"
        ).bind(user_id)
        .fetch_optional(tx)
        .await;

        match res {
            // This won't overflow because total is now capped under 1000
            Ok(Some(total)) => Ok(total.to_u32().unwrap()),
            Ok(None) => Ok(0),
            Err(err) => {
                tracing::error!("{:?}", err);
                Err(anyhow!(""))
            }
        }
    }
}
