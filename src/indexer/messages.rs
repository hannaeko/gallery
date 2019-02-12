use std::path::PathBuf;

use actix_web::actix::{Message, Addr};

use super::IndexerActor;
use crate::error::GalleryError;

pub struct StartIndexing {
    pub storage_path: String,
    pub indexer: Addr<IndexerActor>,
}

pub struct IndexDirectory {
    pub path: PathBuf,
    pub parent: String,
    pub indexer: Addr<IndexerActor>,
}

pub struct IndexFile {
    pub path: PathBuf,
    pub parent: String,
    pub indexer: Addr<IndexerActor>,
}

impl Message for StartIndexing {
    type Result = Result<(), GalleryError>;
}

impl Message for IndexDirectory {
    type Result = Result<(), GalleryError>;
}

impl Message for IndexFile {
    type Result = Result<(), GalleryError>;
}
