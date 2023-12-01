use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug)]
#[derive(FromRow)]
#[derive(Serialize, Deserialize)]
pub struct TransactionEntity {
    pub id: u32,
    pub user_id: u32,
    pub amount: u32,
    pub description: String,
}
