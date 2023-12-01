use crate::adopter::Adopter;
use crate::service::transaction as service;
use crate::model::transaction::form;

use axum::{
    response::IntoResponse,
    Json,
    extract::{
        State, TypedHeader
    },
};
use hyper::HeaderMap;
use sqlx::MySqlPool;

pub async fn add_transaction(
    headers: HeaderMap,
    State(pool): State<MySqlPool>,
    Json(payload): Json<form::AddTransactionForm>,
) -> impl IntoResponse {
    // TODO: This should be moved to layer when auth process is generalized.
    tracing::info!("`add_transaction` called. UserId: {}", payload.user_id);

    tracing::info!("{:?}", headers);

    let adopter = Adopter::new(pool).await;

    service::add_transaction(adopter, payload).await
}
