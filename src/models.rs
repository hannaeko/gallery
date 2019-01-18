use actix_web::{Responder, HttpRequest, HttpResponse, Error};
use std::path::PathBuf;
use std::io;

use crate::config;

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
        Ok(Album::new(name))
    }
}

impl Responder for Album {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, _req: &HttpRequest<S>) -> Result<Self::Item, Self::Error> {
        Ok(HttpResponse::Ok().content_type("text/plain").body(format!{"{:?}", self}))
    }
}

#[derive(Debug)]
pub struct Photo {
    name: String
}

impl Photo {
    pub fn new(name: String) -> Self {
        Photo {
            name,
        }
    }

    pub fn from_path(path: PathBuf) -> io::Result<Self> {
        let name = path.file_name()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Photo not found"))?
            .to_os_string()
            .into_string()
            .unwrap();

        Ok(Photo::new(name))
    }
}

impl Responder for Photo {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, _req: &HttpRequest<S>) -> Result<Self::Item, Self::Error> {
        Ok(HttpResponse::Ok().content_type("text/plain").body(format!("{:?}", self)))
    }
}
#[derive(Debug)]
pub struct AlbumThumbnail {
    name: String
}

#[derive(Debug)]
pub struct PhotoThumbnail {
    name: String
}
