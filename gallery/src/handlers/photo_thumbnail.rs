use actix_web::actix::Handler;
use diesel;
use diesel::prelude::*;

use crate::models::db::{DbExecutor};
use crate::models::photo_thumbnail::{PhotoThumbnail, GetPhotosThumbnail};
use crate::error::GalleryError;

impl Handler<GetPhotosThumbnail> for DbExecutor {
    type Result = Result<Vec<PhotoThumbnail>, GalleryError>;

    fn handle(&mut self, msg: GetPhotosThumbnail, _ctx: &mut Self::Context) -> Self::Result {
        use crate::models::schema::photos::dsl::*;

        let thumbnails = photos.filter(album_id.eq(msg.parent_album_id))
            .select((name, creation_date))
            .order(name.asc())
            .load::<PhotoThumbnail>(&self.conn.get().unwrap())?;

        Ok(thumbnails)
    }
}
