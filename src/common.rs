use actix_web::actix::Addr;

use crate::config::Config;
use crate::models::db::DbExecutor;

pub struct AppState {
    pub config: Config,
    pub db: Addr<DbExecutor>,
}
