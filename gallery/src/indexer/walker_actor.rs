use std::fs;
use std::path::PathBuf;

use actix_web::actix::{Actor, Addr, Arbiter, Context, Handler, Message};
use futures::future::{Future, join_all};

use crate::models::album::{CreateAlbum, GetAlbumId, GetRootAlbumId};
use crate::models::db::DbExecutor;
use crate::config::Config;
use crate::error::GalleryError;
use crate::utils;
use crate::indexer::indexer_actor::{IndexerActor, IndexFile};


pub struct WalkerActor {
    db: Addr<DbExecutor>,
    indexer: Addr<IndexerActor>,
    config: Config,
}

impl Actor for WalkerActor {
    type Context = Context<Self>;
}

impl WalkerActor {
    pub fn init(db: Addr<DbExecutor>, indexer: Addr<IndexerActor>, config: Config) -> Addr<Self> {
        Arbiter::start(move |_ctx| {
            WalkerActor {
                db,
                indexer,
                config
            }
        })
    }
    fn index_children(&self, path: PathBuf, parent: String) -> Result<(), GalleryError> {
        let mut children_future = Vec::new();
        let mut directories = Vec::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                directories.push(path);
            } else if path.is_file() {
                children_future.push(self.indexer.send(IndexFile {
                    path,
                    parent: parent.clone(),
                }));
            }
        }
        join_all(children_future).wait()?.iter()
            .filter_map(|r| r.as_ref().err())
            .for_each(|err| error!("Error during indexing, {}", err));
        for dir in directories {
            self.index_directory(dir, parent.clone())?;
        }
        Ok(())
    }

    fn index_directory(&self, path: PathBuf, parent: String) -> Result<(), GalleryError> {
        info!("Indexing directory {:?}", path);

        let name =  utils::get_file_name_string(&path)?;

        let album_id_opt = self.db.send(GetAlbumId {
            name: name.clone(),
            parent_album_id: parent.clone()
        }).wait()??;

        let album_id = match album_id_opt {
            Some(id) => id,
            None => self.create_album(name, Some(parent))?
        };

        self.index_children(path, album_id)?;
        Ok(())
    }

    fn create_album(&self, name: String, parent_album_id: Option<String>) -> Result<String, GalleryError> {
        let id = self.db.send(CreateAlbum {
            name,
            parent_album_id,
        }).wait()??;
        Ok(id)
    }
}

pub struct StartWalking;

impl Message for StartWalking {
    type Result = Result<(), GalleryError>;
}

impl Handler<StartWalking> for WalkerActor {
    type Result = Result<(), GalleryError>;

    fn handle(&mut self, _msg: StartWalking, _ctx: &mut Self::Context) -> Self::Result {
        info!("Starting building index.");

        let storage_path = fs::canonicalize(&self.config.storage_path)?;

        let root_id_opt = self.db.send(GetRootAlbumId).wait()??;

        let root_id = match root_id_opt {
            Some(id) => id,
            None => self.create_album(self.config.gallery_name.clone(), None)?
        };

        self.index_children(storage_path, root_id)?;
        info!("Done!");
        Ok(())
    }
}
