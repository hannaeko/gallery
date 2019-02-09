use actix_web::actix::{Actor, Addr, SyncContext, SyncArbiter};

use crate::models::db::DbExecutor;

pub struct IndexerActor {
    db_addr: Addr<DbExecutor>,
}

impl Actor for IndexerActor {
    type Context = SyncContext<Self>;
}

pub fn init(db_addr: Addr<DbExecutor>) -> Addr<IndexerActor> {
    SyncArbiter::start(2, move || {
        IndexerActor { db_addr: db_addr.clone() }
    })
}
