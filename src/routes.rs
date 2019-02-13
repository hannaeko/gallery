use std::path::PathBuf;

use actix_web::{HttpRequest, Result, Either, fs::NamedFile};
use futures::prelude::*;

use crate::utils::*;
use crate::models::{AlbumTemplate, Photo, PhotoThumbnail, ThumbnailSize};
use crate::models::album::GetAlbum;
use crate::models::album_thumbnail::GetAlbumsThumbnail;
use crate::models::photo_thumbnail::GetPhotosThumbnail;
use crate::error::GalleryError;
use crate::common::AppState;


pub fn gallery_route(req: &HttpRequest<AppState>) -> Result<Either<AlbumTemplate, Photo>> {
    let path = req.match_info().query("path")?;
    let state = req.state();
    if is_path_album(&path, &state.config) {
        let album = state.db.send(GetAlbum { path: path.clone() })
            .map_err(|e| GalleryError::InternalError(Box::new(e)))
            .flatten()
            .and_then(|album| {
                let albums_tn_future = state.db.send(GetAlbumsThumbnail {
                    parent_album_id: album.id.clone()
                });
                let photos_tn_future = state.db.send(GetPhotosThumbnail {
                    parent_album_id: album.id.clone()
                });
                albums_tn_future
                    .join3(photos_tn_future, Ok(album))
                    .map_err(|e| GalleryError::InternalError(Box::new(e)))
                    .and_then(|(albums, photos, album)| {
                        match (albums, photos) {
                            (Ok(albums), Ok(photos)) => {
                                let album_path = if path == PathBuf::from("") {
                                    "".to_string()
                                } else {
                                    PathBuf::from("/").join(&path).to_str().unwrap().to_string()
                                };

                                Ok(AlbumTemplate {
                                    name: album.name,
                                    breadcrumb: Vec::new(),
                                    album_path: album_path,
                                    albums: albums,
                                    photos: photos,
                                })
                            },
                            (Err(e), _) | (_, Err(e)) => Err(e)
                        }
                    })
            })

            .wait()?;
        println!("{:#?}", album);
        //Ok(Either::A(AlbumTemplate::from_path(path, &state.config)?))
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
