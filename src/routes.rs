use actix_web::{HttpRequest, Result, Either, fs::NamedFile};
use futures::prelude::*;

use crate::utils::*;
use crate::models::{AlbumTemplate, Photo, PhotoThumbnail, ThumbnailSize};
use crate::models::photo::{GetPhoto, NewPhoto};
use crate::error::GalleryError;
use crate::common::AppState;


pub fn gallery_route(req: &HttpRequest<AppState>) -> Result<Either<AlbumTemplate, Photo>> {
    let path: std::path::PathBuf = req.match_info().query("path")?;
    let state = req.state();
    let r = AlbumTemplate::get(path.clone(), state.db.clone())
        .map(|album| Either::A(album))
        .or_else(|err| -> Box<Future<Item = Either<AlbumTemplate, Photo>, Error = GalleryError>> {
            match err {
                 GalleryError::AlbumNotFound {
                     missing_segments,
                     ref last_album,
                     ref current_breadcrumb,
                 } if missing_segments == 1 => {
                    let name = path.file_name().unwrap().to_os_string().into_string().unwrap();

                    let res = Photo::get(name,
                        last_album.to_owned(),
                        current_breadcrumb.clone(),
                        state.db.clone(),
                    ).map(|photo| Either::B(photo));
                    Box::new(res)
                },
                e => Box::new(futures::future::err(e))
            }
        }).wait()?;
    println!("{:#?}", r);
    Ok(r)
    /*if is_path_album(&path, &state.config) {
        let album = AlbumTemplate::get(path, state.db.clone()).wait()?;
        Ok(Either::A(album))
    } else {
        Ok(Either::B(Photo::from_path(path, &state.config)?))
    }*/
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
