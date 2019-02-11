use std::io;
use std::fs;
use std::path::PathBuf;
use futures::prelude::*;

use actix_web::actix::{Actor, Addr, SyncContext, SyncArbiter, Handler};

use crate::models::db::DbExecutor;
use crate::models::album::{CreateAlbum, GetAlbumId, GetRootAlbumId};
use crate::indexer::messages::*;
use crate::error::GalleryError;

pub struct IndexerActor {
    db: Addr<DbExecutor>,
}

impl Actor for IndexerActor {
    type Context = SyncContext<Self>;
}

pub fn init(db_addr: Addr<DbExecutor>) -> Addr<IndexerActor> {
    SyncArbiter::start(2, move || {
        IndexerActor { db: db_addr.clone() }
    })
}
impl IndexerActor {
    fn index_children(path: PathBuf, parent: String, indexer: Addr<IndexerActor>) -> io::Result<()> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                indexer.do_send(IndexDirectory {
                    path,
                    parent: parent.clone(),
                    indexer: indexer.clone()
                });
            }
        }
        Ok(())
    }

    fn create_album(&mut self, name: String, parent_album_id: Option<String>) -> Result<String, GalleryError> {
        self.db.send(CreateAlbum {
            name,
            parent_album_id,
        }).wait()
            .map_err(|_| GalleryError::IndexingError)?
            .map_err(|_| GalleryError::IndexingError)
    }
}

impl Handler<IndexDirectory> for IndexerActor {
    type Result = Result<(), GalleryError>;

    fn handle(&mut self, msg: IndexDirectory, _ctx: &mut Self::Context) -> Self::Result {
        debug!("Indexing directory {:?}", msg.path);
        let name = msg.path.file_name().unwrap().to_os_string().into_string().unwrap();

        let album_id_opt = self.db.send(GetAlbumId {
            name: name.clone(),
            parent_album_id: msg.parent.clone()
        }).wait()
            .map_err(|_| GalleryError::IndexingError)?
            .map_err(|_| GalleryError::IndexingError)?;

        let album_id = match album_id_opt {
            Some(id) => id,
            None => self.create_album(name, Some(msg.parent))?
        };

        IndexerActor::index_children(msg.path, album_id, msg.indexer)?;

        Ok(())
    }
}

impl Handler<StartIndexing> for IndexerActor {
    type Result = Result<(), GalleryError>;

    fn handle(&mut self, msg: StartIndexing, _ctx: &mut Self::Context) -> Self::Result {
        debug!("Starting building index.");

        let storage_path = fs::canonicalize(msg.storage_path)?;

        let root_id_opt = self.db.send(GetRootAlbumId).wait()
            .map_err(|_| GalleryError::IndexingError)?
            .map_err(|_| GalleryError::IndexingError)?;

        let root_id = match root_id_opt {
            Some(id) => id,
            None => self.create_album(String::from(""), None)?
        };
        println!("{}", root_id);
        IndexerActor::index_children(storage_path, root_id, msg.indexer)?;

        Ok(())
    }
}
