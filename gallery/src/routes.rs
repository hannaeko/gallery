use actix_web::{HttpRequest, Result, Either, fs::NamedFile, AsyncResponder, State};
use futures::future::{self, Future};

use crate::utils::*;
use crate::models::{AlbumTemplate, PhotoTemplate, PhotoThumbnail, ThumbnailSize};
use crate::error::GalleryError;
use crate::common::AppState;


pub fn gallery_route((req, state): (HttpRequest<AppState>, State<AppState>))
    -> Box<Future<Item = Either<AlbumTemplate, PhotoTemplate>, Error = GalleryError>>
{
    let path: std::path::PathBuf = req.match_info().query("path").unwrap();
    AlbumTemplate::get(path.clone(), state.db.clone())
        .map(|album| Either::A(album))
        .or_else(move |err| -> Box<Future<Item = Either<AlbumTemplate, PhotoTemplate>, Error = GalleryError>> {
            match err {
                 GalleryError::AlbumNotFound {
                     missing_segments,
                     ref last_album,
                     ref current_breadcrumb,
                 } if missing_segments == 1 => {
                    let name = match get_file_name_string(path) {
                        Ok(name) => name,
                        Err(e) => return Box::new(future::err(e))
                    };

                    let res = PhotoTemplate::get(name,
                        last_album.to_owned(),
                        current_breadcrumb.clone(),
                        state.db.clone(),
                    ).map(|photo| Either::B(photo));
                    Box::new(res)
                },
                e => Box::new(future::err(e))
            }
        }).responder()
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
