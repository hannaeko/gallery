use actix_web::actix::Handler;
use uuid;
use diesel;
use diesel::prelude::*;
use diesel::result::Error as DieselError;

use crate::models::db::{DbExecutor};
use crate::models::album::{Album, AlbumResult, CreateAlbum, GetAlbum, GetAlbumId, GetRootAlbumId};
use crate::error::GalleryError;

impl Handler<CreateAlbum> for DbExecutor {
    type Result = Result<String, GalleryError>;

    fn handle(&mut self, msg: CreateAlbum, _ctx: &mut Self::Context) -> Self::Result {
        use crate::models::schema::albums;

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
        use crate::models::schema::albums::dsl::*;

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
        use crate::models::schema::albums::dsl::*;

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
        use crate::models::schema::albums::dsl::*;

        let album_id = albums
            .filter(parent_album_id.is_null())
            .select(id)
            .load::<String>(&self.conn.get().unwrap())?
            .pop();

        Ok(album_id)
    }
}
