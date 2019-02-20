use std::path::PathBuf;

use actix_web::{HttpRequest, HttpResponse, Result, Either, fs::NamedFile, AsyncResponder, State, HttpMessage};
use futures::future::{self, Future};

use crate::utils::*;
use crate::models::{Album, AlbumTemplate, Photo, PhotoTemplate, PhotoThumbnail};
use crate::models::job::{GetJobs, CreateJob, JobsTemplate};
use crate::error::{GalleryError, GalleryInternalError};
use crate::common::AppState;
use crate::indexer::walker_actor::StartWalking;


pub fn gallery_route((req, state): (HttpRequest<AppState>, State<AppState>))
    -> Box<Future<Item = Either<AlbumTemplate, PhotoTemplate>, Error = GalleryError>>
{
    let path: PathBuf = future_try!(req.match_info().query("path").map_err(GalleryInternalError));
    AlbumTemplate::get(path.clone(), state.db.clone())
        .map(|album| Either::A(album))
        .or_else(move |err| -> Box<Future<Item = Either<AlbumTemplate, PhotoTemplate>, Error = GalleryError>> {
            match err {
                 GalleryError::AlbumNotFound {
                     missing_segments,
                     ref last_album,
                     ref current_breadcrumb,
                 } if missing_segments == 1 => {
                    let name = future_try!(get_file_name_string(path));

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

pub fn thumbnail_route((req, state): (HttpRequest<AppState>, State<AppState>))
    -> Box<Future<Item = NamedFile, Error = GalleryError>>
{
    let path: PathBuf = future_try!(req.match_info().query("path").map_err(GalleryInternalError));
    let thumbnail_size: String = future_try!(req.match_info().query("thumbnail_size").map_err(GalleryInternalError));

    let name = future_try!(get_file_name_string(&path).map_err(GalleryError::from));

    let thumbnail_config = future_try!(state.config.thumbnails.get(&thumbnail_size).ok_or(GalleryError::NotFound)).clone();

    let thumbnail_path = future_try!(path.parent().ok_or(GalleryError::NotFound)).to_path_buf();
    let cache_path = state.config.cache_path.clone();

    Album::get(thumbnail_path, state.db.clone())
        .and_then(move |result| {
            Photo::get(name, result.album.id, state.db.clone())
        })
        .and_then(move |photo| -> Box<Future<Item = NamedFile, Error = GalleryError>> {
            let res = NamedFile::open(PhotoThumbnail::get_image_path(
                &photo.hash,
                &thumbnail_config,
                cache_path
            ));
            match res {
                Ok(res) => Box::new(future::result(Ok(res))),
                Err(e) => Box::new(future::err(GalleryError::from(e)))
            }
        })
        .responder()
}

pub fn full_photo_route(req: &HttpRequest<AppState>) -> Result<NamedFile> {
    let state = req.state();
    let path = get_album_canonical_path(req.match_info().query("path")?, &state.config);

    Ok(NamedFile::open(path)?)
}

pub fn get_jobs_route((_req, state): (HttpRequest<AppState>, State<AppState>)) -> impl Future<Item = JobsTemplate, Error = GalleryError> {
    state.db.send(GetJobs).from_err::<GalleryError>()
        .flatten()
        .and_then(|jobs| {
            Ok(JobsTemplate { jobs })
        })
}

pub fn post_jobs_route((req, state): (HttpRequest<AppState>, State<AppState>)) -> Box<Future<Item = HttpResponse, Error = GalleryError>> {
    let walker_addr = state.walker.clone();

    req.urlencoded::<CreateJob>()
        .map_err(|e| GalleryError::ActixError(e.into()))
        .and_then(|create_job| {
            match create_job.name.as_ref() {
                "index_gallery" => Ok(create_job),
                _ => Err(GalleryError::InvalidForm("\"name\" field value is invalid."))
            }
        })
        .and_then(move |create_job| {
            state.db.send(create_job)
                .from_err::<GalleryError>()
                .flatten()
        })
        .and_then(move |job_id| {
            walker_addr.do_send(StartWalking { job_id: job_id.clone() });
            future::ok(HttpResponse::Created().body(job_id).into())
        })
        .responder()
}
