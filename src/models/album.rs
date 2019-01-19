use std::io;
use std::fs;
use std::path::PathBuf;

use crate::models::*;
use crate::config;

use actix_web::{Responder, HttpRequest, HttpResponse, Error};

#[derive(Debug)]
pub struct Album {
    name: String,
    albums: Vec<AlbumThumbnail>,
    photos: Vec<PhotoThumbnail>
}

impl Album {
    pub fn new(name: String) -> Self {
        Album {
            name,
            albums: Vec::new(),
            photos: Vec::new(),
        }
    }

    pub fn from_path(path: PathBuf) -> io::Result<Self> {
        let name = if let Some(file_name) = path.file_name() {
            file_name.to_os_string().into_string().unwrap()
        } else {
            String::from(config::GALLERY_NAME)
        };

        let mut album = Album::new(name);

        for entry in fs::read_dir(path)? {
            let sub_path = entry?.path();
            if sub_path.is_dir() {
                album.albums.push(AlbumThumbnail::from_path(sub_path)?);
            } else if sub_path.is_file() {
                album.photos.push(PhotoThumbnail::from_path(sub_path)?);
            }
        }

        Ok(album)
    }
}

impl Responder for Album {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, _req: &HttpRequest<S>) -> Result<Self::Item, Self::Error> {
        Ok(HttpResponse::Ok().content_type("text/plain").body(format!{"{:?}", self}))
    }
}
