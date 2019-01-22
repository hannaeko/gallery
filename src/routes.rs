use actix_web::{HttpRequest, Result, Either, fs::NamedFile};

use crate::utils::*;
use crate::models::{Album, Photo, PhotoThumbnail, ThumbnailSize};
use crate::config::Config;


pub fn gallery_route(req: &HttpRequest<Config>) -> Result<Either<Album, Photo>> {
    let path = req.match_info().query("path")?;
    if is_path_album(&path, req.state()) {
        Ok(Either::A(Album::from_path(path, req.state())?))
    } else {
        Ok(Either::B(Photo::from_path(path, req.state())?))
    }
}

pub fn small_thumbnail_route(req: &HttpRequest<Config>) -> Result<NamedFile> {
    let path = get_album_canonical_path(req.match_info().query("path")?, req.state());
    let config = req.state();

    Ok(NamedFile::open(PhotoThumbnail::get_image(
        path,
        ThumbnailSize::Small,
        config
    )?)?)
}

pub fn full_photo_route(req: &HttpRequest<Config>) -> Result<NamedFile> {
    let path = get_album_canonical_path(req.match_info().query("path")?, req.state());

    Ok(NamedFile::open(path)?)
}
