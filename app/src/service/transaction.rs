use crate::context::response::{internal_server_error, payment_required, created};
use crate::adopter::Adopter;
use crate::model::transaction::form;

use axum::response::IntoResponse;

const FREE_TRANSACTION_LIMIT: u32 = 1000;

pub async fn add_transaction<'a>(
    adopter: Adopter,
    form: form::AddTransactionForm,
) -> impl IntoResponse {
    // TODO: 
    // - Track retry count to avoid causing loop infinitely in some case.
    // - Make this retry feature a macro or build-in into trait `service`.
    loop {
        let mut tx = adopter.start_transaction().await;

        let res = adopter.user.repository.get_total_transaction_for_update(
            &mut tx,
            &form.user_id,
        ).await;

        match res {
            Ok(total)
                if (total + form.amount) <= FREE_TRANSACTION_LIMIT
                    => (tracing::debug!("Total transaction: {}", total)),
            Ok(_) => return payment_required(),
            Err(_) => continue
        };

        let res = adopter.transaction.repository.add_transaction(
            &mut tx,
            &form,
        ).await;

        if let Err(_) = res {
            continue
        }

        let res = tx.commit().await;

        return match res {
            Ok(_) => created(),
            Err(_) => internal_server_error()
        }
    }
}
