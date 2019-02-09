use actix_web::actix::Addr;

use crate::config::Config;
use crate::models::db::DbExecutor;
use crate::indexer::IndexerActor;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub db: Addr<DbExecutor>,
    pub index: Addr<IndexerActor>,
}
