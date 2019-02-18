use actix_web::actix::Handler;
use uuid;
use diesel;
use diesel::prelude::*;

use crate::models::db::{DbExecutor};
use crate::models::photo::{Photo, CreatePhoto, GetPhoto, GetPhotoId, GetAdjacentPhotos};
use crate::error::GalleryError;

impl Handler<CreatePhoto> for DbExecutor {
    type Result = Result<String, GalleryError>;

    fn handle(&mut self, msg: CreatePhoto, _ctx: &mut Self::Context) -> Self::Result {
        use crate::models::schema::photos;

        let uuid = uuid::Uuid::new_v4().to_string();

        let mut new_photo = msg.photo;
        new_photo.id = uuid;

        diesel::insert_into(photos::table)
            .values(&new_photo)
            .execute(&self.conn.get().unwrap())?;

        debug!("Inserting new photo in database {} -> {}", new_photo.id, new_photo.name);

        Ok(new_photo.id)
    }
}

impl Handler<GetPhoto> for DbExecutor {
    type Result = Result<Photo, GalleryError>;

    fn handle(&mut self, msg: GetPhoto, _ctx: &mut Self::Context) -> Self::Result {
        use crate::models::schema::photos::dsl::*;

        let photo = photos
            .filter(album_id.eq(&msg.album_id))
            .filter(name.eq(&msg.name))
            .first::<Photo>(&self.conn.get().unwrap())?;
        Ok(photo)
    }
}

impl Handler<GetPhotoId> for DbExecutor {
    type Result = Result<Option<String>, GalleryError>;

    fn handle(&mut self, msg: GetPhotoId, _ctx: &mut Self::Context) -> Self::Result {
        use crate::models::schema::photos::dsl::*;

        let photo_id = photos
            .filter(album_id.eq(&msg.album_id))
            .filter(name.eq(&msg.name))
            .select(id)
            .load::<String>(&self.conn.get().unwrap())?
            .pop();

        Ok(photo_id)
    }
}

impl Handler<GetAdjacentPhotos> for DbExecutor {
    type Result = Result<(Option<String>, Option<String>), GalleryError>;

    fn handle(&mut self, msg: GetAdjacentPhotos, _ctx: &mut Self::Context) -> Self::Result {
        use crate::models::schema::photos::dsl::*;
        let conn = self.conn.get().unwrap();

        let previous = photos.filter(album_id.eq(&msg.album_id))
            .filter(name.lt(&msg.name))
            .select(name)
            .order(name.desc())
            .limit(1)
            .load::<String>(&conn)?
            .pop();

        let next = photos.filter(album_id.eq(&msg.album_id))
            .filter(name.gt(&msg.name))
            .select(name)
            .order(name.asc())
            .limit(1)
            .load::<String>(&conn)?
            .pop();

        Ok((previous, next))
    }
}
