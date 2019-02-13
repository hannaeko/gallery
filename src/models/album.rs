use std::path::PathBuf;

use askama::Template;
use actix_web::actix::{Addr, Message};
use futures::prelude::*;

use super::db::DbExecutor;
use super::schema::albums;
use super::album_thumbnail::{AlbumThumbnail, GetAlbumsThumbnail};
use super::photo_thumbnail::{PhotoThumbnail, GetPhotosThumbnail};
use crate::utils;
use crate::config::Config;
use crate::error::GalleryError;

#[derive(Debug, Template)]
#[template(path = "album.html")]
pub struct AlbumTemplate {
    pub name: String,
    pub breadcrumb: Vec<(String, String)>,
    pub album_path: String,
    pub albums: Vec<AlbumThumbnail>,
    pub photos: Vec<PhotoThumbnail>
}

#[derive(Debug, Insertable, Identifiable, Queryable, Associations)]
#[belongs_to(Album, foreign_key = "parent_album_id")]
#[table_name = "albums"]
pub struct Album {
    pub id: String,
    pub name: String,
    pub parent_album_id: Option<String>,
}

impl AlbumTemplate {
    pub fn get(path: PathBuf, db: Addr<DbExecutor>, config: Config) -> impl Future<Item = Self, Error = GalleryError> {
        db.send(GetAlbum { path: path.clone() })
            .map_err(|e| GalleryError::InternalError(Box::new(e)))
            .flatten()
            .and_then(move |album| {
                let albums_tn_future = db.send(GetAlbumsThumbnail {
                    parent_album_id: album.id.clone()
                });
                let photos_tn_future = db.send(GetPhotosThumbnail {
                    parent_album_id: album.id.clone()
                });
                albums_tn_future
                    .join3(photos_tn_future, Ok(album))
                    .map_err(|e| GalleryError::InternalError(Box::new(e)))
                    .and_then(move |(albums, photos, album)| {
                        match (albums, photos) {
                            (Ok(albums), Ok(photos)) => {
                                let album_path = if path == PathBuf::from("") {
                                    "".to_string()
                                } else {
                                    PathBuf::from("/").join(&path).to_str().unwrap().to_string()
                                };

                                Ok(AlbumTemplate {
                                    name: album.name,
                                    breadcrumb: utils::get_breadcrumb(&path, &config),
                                    album_path: album_path,
                                    albums: albums,
                                    photos: photos,
                                })
                            },
                            (Err(e), _) | (_, Err(e)) => Err(e)
                        }
                    })
            })
    }
}

pub struct CreateAlbum {
    pub name: String,
    pub parent_album_id: Option<String>,
}

pub struct GetAlbum {
    pub path: PathBuf,
}

pub struct GetAlbumId {
    pub name: String,
    pub parent_album_id: String,
}

pub struct GetRootAlbumId;

impl Message for CreateAlbum {
    type Result = Result<String, GalleryError>;
}

impl Message for GetAlbum {
    type Result = Result<Album, GalleryError>;
}

impl Message for GetAlbumId {
    type Result = Result<Option<String>, GalleryError>;
}

impl Message for GetRootAlbumId {
    type Result = Result<Option<String>, GalleryError>;
}
