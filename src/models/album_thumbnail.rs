use actix_web::actix::Message;

use crate::error::GalleryError;

#[derive(Debug, Queryable)]
pub struct AlbumThumbnail {
    pub name: String
}

pub struct GetAlbumsThumbnail {
    pub parent_album_id: String,
}

impl Message for GetAlbumsThumbnail {
    type Result = Result<Vec<AlbumThumbnail>, GalleryError>;
}
