use actix_web::actix::{Actor, Addr, SyncContext, SyncArbiter, Handler};
use uuid;

use diesel;
use diesel::result::Error as DieselError;
use diesel::prelude::*;
use diesel::r2d2::{Pool, ConnectionManager};

use super::album::{Album, AlbumResult, CreateAlbum, GetAlbum, GetAlbumId, GetRootAlbumId};
use super::photo::{NewPhoto, CreatePhoto, GetPhoto, GetPhotoId, GetAdjacentPhotos};
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
    type Result = Result<AlbumResult, GalleryError>;

    fn handle(&mut self, msg: GetAlbum, _ctx: &mut Self::Context) -> Self::Result {
        use super::schema::albums::dsl::*;

        let conn = self.conn.get().unwrap();
        let albums_names: Vec<_> = msg.path.iter().map(|e| e.to_str().unwrap()).collect();
        let mut current_album = albums
            .filter(parent_album_id.is_null())
            .first::<Album>(&conn)?;

        let mut breadcrumb: Vec<(String, String)> = vec![(String::from("/"), current_album.name.clone())];
        let mut current_path = String::from("");
        let mut missing_segments = albums_names.len() as u8;

        for album_name in albums_names {
            let result = Album::belonging_to(&current_album)
                .filter(name.eq(album_name))
                .first::<Album>(&conn);

            current_album = match result {
                Ok(album) => album,
                Err(DieselError::NotFound) => return Err(GalleryError::AlbumNotFound {
                    missing_segments,
                    last_album: current_album.id,
                    current_breadcrumb: breadcrumb,
                }),
                Err(e) => return Err(GalleryError::DbError(e))
            };

            current_path.push_str("/");
            current_path.push_str(&current_album.name);

            breadcrumb.push((current_path.clone(), current_album.name.clone()));
            missing_segments -= 1;
        }

        breadcrumb.pop();

        Ok(AlbumResult {
            album: current_album,
            breadcrumb: breadcrumb,
        })
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

impl Handler<GetPhoto> for DbExecutor {
    type Result = Result<NewPhoto, GalleryError>;

    fn handle(&mut self, msg: GetPhoto, _ctx: &mut Self::Context) -> Self::Result {
        use super::schema::photos::dsl::*;

        let photo = photos
            .filter(album_id.eq(&msg.album_id))
            .filter(name.eq(&msg.name))
            .first::<NewPhoto>(&self.conn.get().unwrap())?;
        Ok(photo)
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

impl Handler<GetAdjacentPhotos> for DbExecutor {
    type Result = Result<(Option<String>, Option<String>), GalleryError>;

    fn handle(&mut self, msg: GetAdjacentPhotos, _ctx: &mut Self::Context) -> Self::Result {
        use super::schema::photos::dsl::*;
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

impl Handler<GetAlbumsThumbnail> for DbExecutor {
    type Result = Result<Vec<AlbumThumbnail>, GalleryError>;

    fn handle(&mut self, msg: GetAlbumsThumbnail, _ctx: &mut Self::Context) -> Self::Result {
        use super::schema::albums::dsl::*;

        let thumbnails = albums.filter(parent_album_id.eq(msg.parent_album_id))
            .select((name,))
            .order(name.asc())
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
            .order(name.asc())
            .load::<PhotoThumbnail>(&self.conn.get().unwrap())?;

        Ok(thumbnails)
    }
}
