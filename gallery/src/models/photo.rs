use actix_web::actix::{Addr, Message};
use futures::future::Future;
use askama::Template;
use exif::Tag;

use super::db::DbExecutor;
use super::schema::photos;
use super::helper::ExifExtractor;
use crate::error::GalleryError;

#[derive(Debug, Template)]
#[template(path = "photo.html")]
pub struct PhotoTemplate {
    // Keeping name here because currently required for breadcrumb templating
    name: String,

    photo: Photo,

    breadcrumb: Vec<(String, String)>,
    album_path: String,
    previous_photo: Option<String>,
    next_photo: Option<String>,
}

#[derive(Debug, Insertable, Queryable)]
#[table_name = "photos"]
pub struct Photo {
    pub id: String,
    pub name: String,
    pub album_id: String,

    pub creation_date: Option<String>,
    pub flash: Option<String>,
    pub exposure_time: Option<String>,
    pub aperture: Option<String>,
    pub focal_length: Option<String>,
    pub focal_length_in_35mm: Option<String>,
    pub camera: Option<String>,
}

pub struct CreatePhoto {
    pub name: String,
    pub album_id: String,

    pub creation_date: Option<String>,
    pub flash: Option<String>,
    pub exposure_time: Option<String>,
    pub aperture: Option<String>,
    pub focal_length: Option<String>,
    pub focal_length_in_35mm: Option<String>,
    pub camera: Option<String>,
}

pub struct GetPhoto {
    pub name: String,
    pub album_id: String,
}

pub struct GetPhotoId {
    pub name: String,
    pub album_id: String,
}

pub struct GetAdjacentPhotos {
    pub name: String,
    pub album_id: String,
}

impl Message for CreatePhoto {
    type Result = Result<String, GalleryError>;
}

impl Message for GetPhoto {
    type Result = Result<Photo, GalleryError>;
}

impl Message for GetPhotoId {
    type Result = Result<Option<String>, GalleryError>;
}

impl Message for GetAdjacentPhotos {
    type Result = Result<(Option<String>, Option<String>), GalleryError>;
}

impl PhotoTemplate {
    pub fn get(name: String, album_id: String, breadcrumb: Vec<(String, String)>, db: Addr<DbExecutor>)
        -> impl Future<Item = Self, Error = GalleryError>
    {
        db.send(GetPhoto {
            name: name,
            album_id: album_id,
        }).from_err::<GalleryError>()
            .flatten()
            .and_then(move |photo| {
                let adj_future = db.send(GetAdjacentPhotos {
                    name: photo.name.clone(),
                    album_id: photo.album_id.clone()
                });
                adj_future.join(Ok(photo)).from_err()
            })
            .and_then(move |(res, photo)| {
                match res {
                    Ok((prev, next)) => {
                        let album_path = (&breadcrumb[breadcrumb.len() - 1].0).to_owned();

                        Ok(PhotoTemplate {
                            name: photo.name.clone(),

                            photo: photo,
                            breadcrumb: breadcrumb,
                            album_path: album_path,

                            previous_photo: prev,
                            next_photo: next,
                        })
                    },
                    Err(e) => Err(e)
                }
            })
    }
}

impl ExifExtractor for Photo {
    const TAG_LIST: &'static [Tag] = &[
        Tag::DateTimeOriginal,
        Tag::Flash,
        Tag::ExposureTime,
        Tag::FNumber,
        Tag::FocalLength,
        Tag::FocalLengthIn35mmFilm,
        Tag::Model,
    ];
}
