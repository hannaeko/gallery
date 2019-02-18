use actix_web::actix::{Actor, Addr, SyncContext, SyncArbiter};
use diesel;
use diesel::prelude::*;
use diesel::r2d2::{Pool, ConnectionManager};

pub struct DbExecutor {
    pub conn: Pool<ConnectionManager<SqliteConnection>>,
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub fn init(db_url: String) -> Addr<DbExecutor> {
    let manager = ConnectionManager::<SqliteConnection>::new(db_url);
    let pool = Pool::builder().build(manager).expect("Failed to create pool");

    SyncArbiter::start(1, move || {
        DbExecutor { conn: pool.clone() }
    })
}
