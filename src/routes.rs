use actix_web::{HttpRequest, Result, Either, fs::NamedFile};
use futures::prelude::*;

use crate::utils::*;
use crate::models::{AlbumTemplate, Photo, PhotoThumbnail, ThumbnailSize};
use crate::common::AppState;


pub fn gallery_route(req: &HttpRequest<AppState>) -> Result<Either<AlbumTemplate, Photo>> {
    let path = req.match_info().query("path")?;
    let state = req.state();
    if is_path_album(&path, &state.config) {
        let album = AlbumTemplate::get(path, state.db.clone()).wait()?;
        Ok(Either::A(album))
    } else {
        Ok(Either::B(Photo::from_path(path, &state.config)?))
    }
}

pub fn small_thumbnail_route(req: &HttpRequest<AppState>) -> Result<NamedFile> {
    let state = req.state();
    let path = get_album_canonical_path(req.match_info().query("path")?, &state.config);

    Ok(NamedFile::open(PhotoThumbnail::get_image_path(
        &path,
        ThumbnailSize::Small,
        &state.config
    ))?)
}

pub fn medium_thumbnail_route(req: &HttpRequest<AppState>) -> Result<NamedFile> {
    let state = req.state();
    let path = get_album_canonical_path(req.match_info().query("path")?, &state.config);

    Ok(NamedFile::open(PhotoThumbnail::get_image_path(
        &path,
        ThumbnailSize::Medium,
        &state.config
    ))?)
}

pub fn full_photo_route(req: &HttpRequest<AppState>) -> Result<NamedFile> {
    let state = req.state();
    let path = get_album_canonical_path(req.match_info().query("path")?, &state.config);

    Ok(NamedFile::open(path)?)
}
