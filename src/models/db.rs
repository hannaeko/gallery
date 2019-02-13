use actix_web::actix::{Actor, Addr, SyncContext, SyncArbiter, Handler};
use uuid;

use diesel;
use diesel::prelude::*;
use diesel::r2d2::{Pool, ConnectionManager};

use super::album::{Album, CreateAlbum, GetAlbum, GetAlbumId, GetRootAlbumId};
use super::photo::{NewPhoto, CreatePhoto, GetPhotoId};
use super::album_thumbnail::{AlbumThumbnail, GetAlbumsThumbnail};
use super::photo_thumbnail::{PhotoThumbnail, GetPhotosThumbnail};
use crate::error::GalleryError;

pub struct DbExecutor {
    pub conn: Pool<ConnectionManager<SqliteConnection>>,
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub fn init(db_url: String) -> Addr<DbExecutor> {
    let manager = ConnectionManager::<SqliteConnection>::new(db_url);
    let pool = Pool::builder().build(manager).expect("Failed to create pool");

    SyncArbiter::start(1, move || {
        DbExecutor { conn: pool.clone() }
    })
}

impl Handler<CreateAlbum> for DbExecutor {
    type Result = Result<String, GalleryError>;

    fn handle(&mut self, msg: CreateAlbum, _ctx: &mut Self::Context) -> Self::Result {
        use super::schema::albums;

        let uuid = uuid::Uuid::new_v4().to_string();

        let new_album = Album {
            id: uuid,
            name: msg.name,
            parent_album_id: msg.parent_album_id,
        };

        diesel::insert_into(albums::table)
            .values(&new_album)
            .execute(&self.conn.get().unwrap())?;

        debug!("Inserting new album in database, {} -> \"{}\"", new_album.id, new_album.name);

        Ok(new_album.id)
    }
}

impl Handler<GetAlbum> for DbExecutor {
    type Result = Result<Album, GalleryError>;

    fn handle(&mut self, msg: GetAlbum, _ctx: &mut Self::Context) -> Self::Result {
        use super::schema::albums::dsl::*;

        let conn = self.conn.get().unwrap();
        let albums_names: Vec<_> = msg.path.iter().map(|e| e.to_str().unwrap()).collect();

        let mut current_album = albums
            .filter(parent_album_id.is_null())
            .first::<Album>(&conn)?;

        for album_name in albums_names {
            current_album = Album::belonging_to(&current_album)
                .filter(name.eq(album_name))
                .first::<Album>(&conn)?;
        }

        Ok(current_album)
    }
}

impl Handler<GetAlbumId> for DbExecutor {
    type Result = Result<Option<String>, GalleryError>;

    fn handle(&mut self, msg: GetAlbumId, _ctx: &mut Self::Context) -> Self::Result {
        use super::schema::albums::dsl::*;

        let album_id = albums
            .filter(parent_album_id.eq(&msg.parent_album_id))
            .filter(name.eq(&msg.name))
            .select(id)
            .load::<String>(&self.conn.get().unwrap())?
            .pop();

        Ok(album_id)
    }
}

impl Handler<GetRootAlbumId> for DbExecutor {
    type Result = Result<Option<String>, GalleryError>;

    fn handle(&mut self, _msg: GetRootAlbumId, _ctx: &mut Self::Context) -> Self::Result {
        use super::schema::albums::dsl::*;

        let album_id = albums
            .filter(parent_album_id.is_null())
            .select(id)
            .load::<String>(&self.conn.get().unwrap())?
            .pop();

        Ok(album_id)
    }
}

impl Handler<CreatePhoto> for DbExecutor {
    type Result = Result<String, GalleryError>;

    fn handle(&mut self, msg: CreatePhoto, _ctx: &mut Self::Context) -> Self::Result {
        use super::schema::photos;

        let uuid = uuid::Uuid::new_v4().to_string();

        let new_photo = NewPhoto {
            id: uuid,
            name: msg.name,
            album_id: msg.album_id,

            creation_date: msg.creation_date,
            flash: msg.flash,
            exposure_time: msg.exposure_time,
            aperture: msg.aperture,
            focal_length: msg.focal_length,
            focal_length_in_35mm: msg.focal_length_in_35mm,
            camera: msg.camera,
        };

        diesel::insert_into(photos::table)
            .values(&new_photo)
            .execute(&self.conn.get().unwrap())?;

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
            .load::<String>(&self.conn.get().unwrap())?
            .pop();

        Ok(photo_id)
    }
}

impl Handler<GetAlbumsThumbnail> for DbExecutor {
    type Result = Result<Vec<AlbumThumbnail>, GalleryError>;

    fn handle(&mut self, msg: GetAlbumsThumbnail, _ctx: &mut Self::Context) -> Self::Result {
        use super::schema::albums::dsl::*;

        let thumbnails = albums.filter(parent_album_id.eq(msg.parent_album_id))
            .select((name,))
            .load::<AlbumThumbnail>(&self.conn.get().unwrap())?;

        Ok(thumbnails)
    }
}

impl Handler<GetPhotosThumbnail> for DbExecutor {
    type Result = Result<Vec<PhotoThumbnail>, GalleryError>;

    fn handle(&mut self, msg: GetPhotosThumbnail, _ctx: &mut Self::Context) -> Self::Result {
        use super::schema::photos::dsl::*;

        let thumbnails = photos.filter(album_id.eq(msg.parent_album_id))
            .select((name, creation_date))
            .load::<PhotoThumbnail>(&self.conn.get().unwrap())?;

        Ok(thumbnails)
    }
}
