use std::path::PathBuf;
use std::io;

use actix_web::actix::{Message, Addr};

use crate::indexer::IndexerActor;

pub struct IndexDirectory {
    pub path: PathBuf,
    pub indexer: Addr<IndexerActor>,
}

impl Message for IndexDirectory {
    type Result = io::Result<()>;
}
