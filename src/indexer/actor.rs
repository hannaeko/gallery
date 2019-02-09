use std::io;
use std::fs;

use actix_web::actix::{Actor, Addr, SyncContext, SyncArbiter, Handler};

use crate::models::db::DbExecutor;
use crate::indexer::messages::*;

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

impl Handler<IndexDirectory> for IndexerActor {
    type Result = io::Result<()>;

    fn handle(&mut self, msg: IndexDirectory, _ctx: &mut Self::Context) -> Self::Result {
        debug!("Indexing directory {:?}", msg.path);
        for entry in fs::read_dir(msg.path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                msg.indexer.do_send(IndexDirectory {
                    path,
                    indexer: msg.indexer.clone()
                });
            }
        }
        Ok(())
    }
}
