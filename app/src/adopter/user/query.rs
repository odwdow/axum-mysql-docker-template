use crate::context::{db::ArcPool, adopter::Query};

pub struct UserQuery {
    pool: ArcPool,
}

impl Query for UserQuery {
    fn new(pool: ArcPool) -> Self {
        Self {
            pool,
        }
    }
}

impl UserQuery {
    pub fn find_by_id(&self) {
    }
}
