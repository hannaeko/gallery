use actix_web::{HttpRequest, Result, Either, fs::NamedFile};
use futures::prelude::*;

use crate::utils::*;
use crate::models::{AlbumTemplate, PhotoTemplate, PhotoThumbnail, ThumbnailSize};
use crate::error::GalleryError;
use crate::common::AppState;


pub fn gallery_route(req: &HttpRequest<AppState>) -> Result<Either<AlbumTemplate, PhotoTemplate>> {
    let path: std::path::PathBuf = req.match_info().query("path")?;
    let state = req.state();
    let r = AlbumTemplate::get(path.clone(), state.db.clone())
        .map(|album| Either::A(album))
        .or_else(|err| -> Box<Future<Item = Either<AlbumTemplate, PhotoTemplate>, Error = GalleryError>> {
            match err {
                 GalleryError::AlbumNotFound {
                     missing_segments,
                     ref last_album,
                     ref current_breadcrumb,
                 } if missing_segments == 1 => {
                    let name = path.file_name().unwrap().to_os_string().into_string().unwrap();

                    let res = PhotoTemplate::get(name,
                        last_album.to_owned(),
                        current_breadcrumb.clone(),
                        state.db.clone(),
                    ).map(|photo| Either::B(photo));
                    Box::new(res)
                },
                e => Box::new(futures::future::err(e))
            }
        }).wait()?;
    Ok(r)
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
