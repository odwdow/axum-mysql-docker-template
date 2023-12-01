mod transaction;

use axum::{
    routing::post,
    Router,
};

use crate::context::db::Db;

pub async fn router() -> Router {
    let pool = Db::new().await;
    
    return Router::new()
        .route("/transactions", post(transaction::add_transaction))
        .with_state(pool.0);
}
