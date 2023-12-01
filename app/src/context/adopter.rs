use sqlx::{Transaction, MySql};

use crate::context::db::ArcPool;

pub type Tx = Transaction<'static, MySql>;

pub trait Repository {
    fn new(pool: ArcPool) -> Self;
}

pub trait Query {
    fn new(pool: ArcPool) -> Self;
}

pub struct DBAdopter<R, Q>
    where R: Repository,
        Q: Query {
    pub repository: R,
    pub query: Q,
}

impl<R, Q> DBAdopter<R, Q>
    where R: Repository,
        Q: Query {
    pub fn new(pool: ArcPool) -> Self {
        Self {
            repository: R::new(pool.clone()),
            query: Q::new(pool.clone()),
        }
    }
}
