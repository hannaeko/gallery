use std::convert::From;
use std::io::{self, ErrorKind::{NotFound, PermissionDenied}};

use image::ImageError;
use actix_web::{ResponseError, HttpResponse};

#[derive(Fail, Debug)]
pub enum GalleryError {
    #[fail(display="{}", _0)]
    IoError(io::Error),
    #[fail(display="{}", _0)]
    ImageError(ImageError)
}

impl ResponseError for GalleryError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            GalleryError::IoError(ref io_err) if io_err.kind() == NotFound || io_err.kind() == PermissionDenied => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().finish()
        }
    }
}

impl From<io::Error> for GalleryError {
    fn from(error: io::Error) -> Self {
        GalleryError::IoError(error)
    }
}

impl From<ImageError> for GalleryError {
    fn from(error: ImageError) -> Self {
        match error {
            ImageError::IoError(io_err) => GalleryError::IoError(io_err),
            err => GalleryError::ImageError(err)
        }
    }
}
