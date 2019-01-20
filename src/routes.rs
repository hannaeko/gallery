use actix_web::{HttpRequest, Result, Either, fs::NamedFile};

use crate::utils::*;
use crate::models::{Album, Photo, PhotoThumbnail};


pub fn gallery_route(req: &HttpRequest) -> Result<Either<Album, Photo>> {
    let path = get_album_canonical_path(req.match_info().query("path")?);
    if is_path_album(&path) {
        Ok(Either::A(Album::from_path(path)?))
    } else {
        Ok(Either::B(Photo::from_path(path)?))
    }
}

pub fn small_thumbnail_route(req: &HttpRequest) -> Result<NamedFile> {
    let path = get_album_canonical_path(req.match_info().query("path")?);

    Ok(NamedFile::open(PhotoThumbnail::get_image(path, 250, true)?)?)
}

pub fn full_photo_route(req: &HttpRequest) -> Result<NamedFile> {
    let path = get_album_canonical_path(req.match_info().query("path")?);

    Ok(NamedFile::open(path)?)
}
