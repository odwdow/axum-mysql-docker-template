use anyhow::anyhow;

use crate::context::adopter::{Repository, Tx};
use crate::context::db::ArcPool;
use crate::model::transaction::domain::TransactionEntity;
use crate::model::transaction::form::AddTransactionForm;

pub struct TransactionRepository {
    pool: ArcPool,
}

impl Repository for TransactionRepository {
    fn new(pool: ArcPool) -> Self {
        Self {
            pool,
        }
    }
}

impl TransactionRepository {
    pub async fn add_transaction(
        &self,
        tx: &mut Tx,
        form: &AddTransactionForm,
    ) -> anyhow::Result<()> {
        tracing::debug!("TransactionRepository::add_transaction");

        let res = sqlx::query(
            r"INSERT INTO transactions (user_id, amount, description)
              VALUES (?, ?, ?)"
        ).bind(form.user_id)
        .bind(form.amount)
        .bind(&form.description)
        .execute(tx)
        .await;

        match res {
            Ok(_) => Ok(()),
            Err(err) => {
                tracing::error!("{:?}", err);
                Err(anyhow!(""))
            }
        }
    }

    pub async fn find_by_id(
        &self,
        tx: &mut Tx,
        id: &u32
    ) -> anyhow::Result<TransactionEntity> {
        tracing::debug!("TransactionRepository::find_by_id");

        let res = sqlx::query_as::<_, TransactionEntity>(
            r"SELECT *
              FROM transactions
              WHERE id = ?"
        ).bind(id)
        .fetch_one(tx)
        .await;

        match res {
            Ok(entity) => Ok(entity),
            Err(err) => {
                tracing::error!("{:?}", err);
                Err(anyhow!(""))
            }
        }
    }
}
