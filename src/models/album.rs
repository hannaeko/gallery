use std::fs;
use std::path::PathBuf;

use actix_web::actix::Message;
use diesel::result::Error as DieselError;

use super::*;
use super::schema::albums;
use crate::utils;
use crate::config::Config;
use crate::error::GalleryError;
use askama::Template;

#[derive(Debug, Template)]
#[template(path = "album.html")]
pub struct Album {
    name: String,
    breadcrumb: Vec<(String, String)>,
    album_path: String,
    albums: Vec<AlbumThumbnail>,
    photos: Vec<PhotoThumbnail>
}

#[derive(Debug, Insertable)]
#[table_name = "albums"]
pub struct NewAlbum {
    pub id: String,
    pub name: String,
    pub parent_album_id: Option<String>,
}

pub struct CreateAlbum {
    pub name: String,
    pub parent_album_id: Option<String>,
}

pub struct GetAlbumId {
    pub name: String,
    pub parent_album_id: String,
}

pub struct GetRootAlbumId;

impl Message for CreateAlbum {
    type Result = Result<String, DieselError>;
}

impl Message for GetAlbumId {
    type Result = Result<Option<String>, DieselError>;
}

impl Message for GetRootAlbumId {
    type Result = Result<Option<String>, DieselError>;
}

impl Album {
    pub fn from_path(path: PathBuf, config: &Config) -> Result<Self, GalleryError> {
        let name = if let Some(file_name) = path.file_name() {
            file_name.to_os_string().into_string().unwrap()
        } else {
            config.gallery_name.clone()
        };

        let album_path = if path == PathBuf::from("") {
            "".to_string()
        } else {
            PathBuf::from("/").join(&path).to_str().unwrap().to_string()
        };

        let breadcrumb = utils::get_breadcrumb(&path, config);
        let full_album_path = utils::get_album_canonical_path(path, config);

        let mut albums = Vec::new();
        let mut photos = Vec::new();

        for entry in fs::read_dir(full_album_path)? {
            let sub_path = entry?.path();
            if sub_path.is_dir() {
                albums.push(AlbumThumbnail::from_path(sub_path)?);
            } else if sub_path.is_file() {
                photos.push(PhotoThumbnail::from_path(sub_path)?);
            }
        }

        albums.sort_by(|a, b| a.name.cmp(&b.name));
        photos.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(Album {
            name,
            breadcrumb,
            album_path,
            albums,
            photos,
        })
    }
}
