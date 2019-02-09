use actix_web::actix::{Actor, Addr, SyncContext, SyncArbiter};
use diesel::prelude::SqliteConnection;
use diesel::connection::Connection;

pub struct DbExecutor(SqliteConnection);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub fn init(db_url: String) -> Addr<DbExecutor> {
    SyncArbiter::start(3, move || {
        DbExecutor(SqliteConnection::establish(&db_url)
            .expect("Failed to establish connection to the database"))
    })
}
