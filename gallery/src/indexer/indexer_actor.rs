use std::path::PathBuf;

use actix_web::actix::{Actor, Addr, SyncContext, SyncArbiter, Handler, Message  };
use futures::future::Future;

use crate::models::db::DbExecutor;
use crate::models::photo::{Photo, GetPhotoId, CreatePhoto};
use crate::models::photo_thumbnail::PhotoThumbnail;
use crate::models::helper::ExifExtractor;
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

impl IndexerActor {
    pub fn init(db_addr: Addr<DbExecutor>, config: Config) -> Addr<IndexerActor> {
        SyncArbiter::start(4, move || {
            IndexerActor {
                db: db_addr.clone(),
                config: config.clone(),
            }
        })
    }
}

pub struct IndexFile {
    pub path: PathBuf,
    pub parent: String,
}

impl Message for IndexFile {
    type Result = Result<(), GalleryError>;
}

impl Handler<IndexFile> for IndexerActor {
    type Result = Result<(), GalleryError>;

    fn handle(&mut self, msg: IndexFile, _ctx: &mut Self::Context) -> Self::Result {
        info!("Indexing file {:?}", msg.path);
        let is_valid_extension = msg.path.extension()
            .map(|ext| self.config
                    .allowed_extensions
                    .contains(&ext.to_string_lossy().to_string().to_lowercase()));

        if is_valid_extension != Some(true) {
            warn!("Invalid extension, skipping file");
            return Err(GalleryError::InvalidFileName);
        }

        let name = utils::get_file_name_string(&msg.path)?;

        let photo_id = self.db.send(GetPhotoId {
            name: name.clone(),
            album_id: msg.parent.clone(),
        }).wait()??;

        if let Some(_) = photo_id {
            debug!("Already in index.");
            return Ok(());
        }

        debug!("Generating thumbnails...");
        let hash = Photo::compute_hash(&msg.path)?;

        for (_, thumbnail_config) in &self.config.thumbnails {
            PhotoThumbnail::create_image(&msg.path, &hash, &thumbnail_config, self.config.cache_path.clone())?;
        }

        let mut photo = Photo {
            name,
            hash,
            album_id: msg.parent,
            ..Default::default()
        };

        photo.extract_exif(&msg.path)?;
        photo.camera = photo.camera.map(utils::trim_one_char);

        self.db.send(CreatePhoto { photo: photo }).wait()??;

        Ok(())
    }
}
