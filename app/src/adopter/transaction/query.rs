use crate::context::{db::ArcPool, adopter::Query};

pub struct TransactionQuery {
    pool: ArcPool,
}

impl Query for TransactionQuery {
    fn new(pool: ArcPool) -> Self {
        Self {
            pool,
        }
    }
}

impl TransactionQuery {
    pub fn find_by_id(&self) {
    }
}
