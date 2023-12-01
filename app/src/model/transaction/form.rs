use serde::Deserialize;

#[derive(Debug)]
#[derive(Deserialize)]
pub struct AddTransactionForm {
    pub user_id: u32,
    pub amount: u32,
    pub description: String,
}
