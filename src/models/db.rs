use actix_web::actix::{Actor, Addr, SyncContext, SyncArbiter, Handler};
use uuid;

use diesel;
use diesel::result::Error as DieselError;
use diesel::prelude::*;
use diesel::connection::Connection;
use exif::Tag;

use super::album::{NewAlbum, CreateAlbum, GetAlbumId, GetRootAlbumId};
use super::photo::{Photo, NewPhoto, CreatePhoto, GetPhotoId};
use super::helper::ExifExtractor;
use crate::error::GalleryError;
use crate::utils;

pub struct DbExecutor(SqliteConnection);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub fn init(db_url: String) -> Addr<DbExecutor> {
    SyncArbiter::start(1, move || {
        DbExecutor(SqliteConnection::establish(&db_url)
            .expect("Failed to establish connection to the database"))
    })
}

impl Handler<CreateAlbum> for DbExecutor {
    type Result = Result<String, DieselError>;

    fn handle(&mut self, msg: CreateAlbum, _ctx: &mut Self::Context) -> Self::Result {
        use super::schema::albums;

        let uuid = uuid::Uuid::new_v4().to_string();

        let new_album = NewAlbum {
            id: uuid,
            name: msg.name,
            parent_album_id: msg.parent_album_id,
        };

        diesel::insert_into(albums::table)
            .values(&new_album)
            .execute(&self.0)?;

        debug!("Inserting new album in database, {} -> \"{}\"", new_album.id, new_album.name);

        Ok(new_album.id)
    }
}

impl Handler<GetAlbumId> for DbExecutor {
    type Result = Result<Option<String>, DieselError>;

    fn handle(&mut self, msg: GetAlbumId, _ctx: &mut Self::Context) -> Self::Result {
        use super::schema::albums::dsl::*;

        let album_id = albums
            .filter(parent_album_id.eq(&msg.parent_album_id))
            .filter(name.eq(&msg.name))
            .select(id)
            .load::<String>(&self.0)?
            .pop();

        Ok(album_id)
    }
}

impl Handler<GetRootAlbumId> for DbExecutor {
    type Result = Result<Option<String>, DieselError>;

    fn handle(&mut self, _msg: GetRootAlbumId, _ctx: &mut Self::Context) -> Self::Result {
        use super::schema::albums::dsl::*;

        let album_id = albums
            .filter(parent_album_id.is_null())
            .select(id)
            .load::<String>(&self.0)?
            .pop();

        Ok(album_id)
    }
}

impl Handler<CreatePhoto> for DbExecutor {
    type Result = Result<String, GalleryError>;

    fn handle(&mut self, msg: CreatePhoto, _ctx: &mut Self::Context) -> Self::Result {
        use super::schema::photos;

        let exif_map = Photo::extract_exif(&msg.path)?;
        let uuid = uuid::Uuid::new_v4().to_string();

        let new_photo = NewPhoto {
            id: uuid,
            name: msg.name,
            album_id: msg.album_id,

            creation_date: exif_map[&Tag::DateTimeOriginal].to_owned(),
            flash: exif_map[&Tag::Flash].to_owned(),
            exposure_time: exif_map[&Tag::ExposureTime].to_owned(),
            aperture: exif_map[&Tag::FNumber].to_owned(),
            focal_length: exif_map[&Tag::FocalLength].to_owned(),
            focal_length_in_35mm: exif_map[&Tag::FocalLengthIn35mmFilm].to_owned(),
            camera: utils::trim_one_char(&exif_map[&Tag::Model]),
        };

        diesel::insert_into(photos::table)
            .values(&new_photo)
            .execute(&self.0)?;

        debug!("Inserting new photo in database {} -> {}", new_photo.id, new_photo.name);

        Ok(new_photo.id)
    }
}

impl Handler<GetPhotoId> for DbExecutor {
    type Result = Result<Option<String>, GalleryError>;

    fn handle(&mut self, msg: GetPhotoId, _ctx: &mut Self::Context) -> Self::Result {
        use super::schema::photos::dsl::*;

        let photo_id = photos
            .filter(album_id.eq(&msg.album_id))
            .filter(name.eq(&msg.name))
            .select(id)
            .load::<String>(&self.0)?
            .pop();

        Ok(photo_id)
    }
}
