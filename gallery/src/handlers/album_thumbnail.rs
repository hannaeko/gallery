use actix_web::actix::Handler;
use diesel;
use diesel::prelude::*;

use crate::models::db::{DbExecutor};
use crate::models::album_thumbnail::{AlbumThumbnail, GetAlbumsThumbnail};
use crate::error::GalleryError;

impl Handler<GetAlbumsThumbnail> for DbExecutor {
    type Result = Result<Vec<AlbumThumbnail>, GalleryError>;

    fn handle(&mut self, msg: GetAlbumsThumbnail, _ctx: &mut Self::Context) -> Self::Result {
        use crate::models::schema::albums::dsl::*;

        let thumbnails = albums.filter(parent_album_id.eq(msg.parent_album_id))
            .select((name,))
            .order(name.asc())
            .load::<AlbumThumbnail>(&self.conn.get().unwrap())?;

        Ok(thumbnails)
    }
}
