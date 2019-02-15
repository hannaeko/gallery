use std::io;
use std::fs;
use std::path::PathBuf;

use actix_web::actix::{Actor, Addr, SyncContext, SyncArbiter, Handler};
use exif::Tag;
use futures::future::Future;

use crate::models::db::DbExecutor;
use crate::models::album::{CreateAlbum, GetAlbumId, GetRootAlbumId};
use crate::models::photo::{Photo, GetPhotoId, CreatePhoto};
use crate::models::photo_thumbnail::{PhotoThumbnail, ThumbnailSize};
use crate::models::helper::ExifExtractor;
use crate::indexer::messages::*;
use crate::error::GalleryError;
use crate::utils;
use crate::config::Config;

pub struct IndexerActor {
    db: Addr<DbExecutor>,
    config: Config,
}

impl Actor for IndexerActor {
    type Context = SyncContext<Self>;
}

pub fn init(db_addr: Addr<DbExecutor>, config: Config) -> Addr<IndexerActor> {
    SyncArbiter::start(2, move || {
        IndexerActor {
            db: db_addr.clone(),
            config: config.clone(),
        }
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
            } else if path.is_file() {
                indexer.do_send(IndexFile {
                    path,
                    parent: parent.clone(),
                    indexer: indexer.clone()
                })
            }
        }
        Ok(())
    }

    fn create_album(&mut self, name: String, parent_album_id: Option<String>) -> Result<String, GalleryError> {
        let id = self.db.send(CreateAlbum {
            name,
            parent_album_id,
        }).wait()??;
        Ok(id)
    }
}

impl Handler<StartIndexing> for IndexerActor {
    type Result = Result<(), GalleryError>;

    fn handle(&mut self, msg: StartIndexing, _ctx: &mut Self::Context) -> Self::Result {
        debug!("Starting building index.");

        let storage_path = fs::canonicalize(msg.storage_path)?;

        let root_id_opt = self.db.send(GetRootAlbumId).wait()??;

        let root_id = match root_id_opt {
            Some(id) => id,
            None => self.create_album(self.config.gallery_name.clone(), None)?
        };

        Self::index_children(storage_path, root_id, msg.indexer)?;

        Ok(())
    }
}

impl Handler<IndexDirectory> for IndexerActor {
    type Result = Result<(), GalleryError>;

    fn handle(&mut self, msg: IndexDirectory, _ctx: &mut Self::Context) -> Self::Result {
        debug!("Indexing directory {:?}", msg.path);

        let name =  utils::get_file_name_string(&msg.path)?;

        let album_id_opt = self.db.send(GetAlbumId {
            name: name.clone(),
            parent_album_id: msg.parent.clone()
        }).wait()??;

        let album_id = match album_id_opt {
            Some(id) => id,
            None => self.create_album(name, Some(msg.parent))?
        };

        Self::index_children(msg.path, album_id, msg.indexer)?;

        Ok(())
    }
}

impl Handler<IndexFile> for IndexerActor {
    type Result = Result<(), GalleryError>;

    fn handle(&mut self, msg: IndexFile, _ctx: &mut Self::Context) -> Self::Result {
        debug!("Indexing file {:?}", msg.path);

        let name = utils::get_file_name_string(&msg.path)?;

        let photo_id = self.db.send(GetPhotoId {
            name: name.clone(),
            album_id: msg.parent.clone(),
        }).wait()??;

        if let Some(_) = photo_id {
            return Ok(());
        }

        let mut exif_map = Photo::extract_exif(&msg.path)?;

        self.db.send(CreatePhoto {
            name: name,
            album_id: msg.parent,

            creation_date: exif_map.remove(&Tag::DateTimeOriginal),
            flash: exif_map.remove(&Tag::Flash),
            exposure_time: exif_map.remove(&Tag::ExposureTime),
            aperture: exif_map.remove(&Tag::FNumber),
            focal_length: exif_map.remove(&Tag::FocalLength),
            focal_length_in_35mm: exif_map.remove(&Tag::FocalLengthIn35mmFilm),
            camera: exif_map.remove(&Tag::Model).map(utils::trim_one_char),
        }).wait()??;

        PhotoThumbnail::create_image(&msg.path, ThumbnailSize::Small, &self.config)?;
        PhotoThumbnail::create_image(&msg.path, ThumbnailSize::Medium, &self.config)?;

        Ok(())
    }
}