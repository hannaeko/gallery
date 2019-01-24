use std::convert::From;
use std::io;

use failure::Fail;
use image::ImageError;
use actix_web::{ResponseError, HttpResponse};

#[derive(Fail, Debug)]
pub enum GalleryError {
    #[fail(display="Resource not found")]
    NotFound,
    #[fail(display="Error processing image, {}", _0)]
    ImageError(ImageError),
    #[fail(display="Error processing a file with an invalid file_name")]
    InvalidFileName,
    #[fail(display="{}", _0)]
    InternalError(Box<Fail>)
}

impl ResponseError for GalleryError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            GalleryError::NotFound => HttpResponse::NotFound().content_type("text/html").finish(),
            _ => HttpResponse::InternalServerError().content_type("text/html").finish()
        }
    }
}

impl From<io::Error> for GalleryError {
    fn from(error: io::Error) -> Self {
        match error.kind() {
            io::ErrorKind::NotFound | io::ErrorKind::PermissionDenied => GalleryError::NotFound,
            _ => GalleryError::InternalError(Box::new(error))
        }
    }
}

impl From<ImageError> for GalleryError {
    fn from(error: ImageError) -> Self {
        match error {
            ImageError::IoError(io_err) => GalleryError::from(io_err),
            err => GalleryError::ImageError(err)
        }
    }
}

impl From<std::path::StripPrefixError> for GalleryError {
    fn from(_error: std::path::StripPrefixError) -> Self {
        GalleryError::InvalidFileName
    }
}
