use std::fs;
use std::path::PathBuf;

use actix_web::actix::{Addr, Message};
use futures::prelude::*;
use askama::Template;
use exif::Tag;

use super::db::DbExecutor;
use super::schema::photos;
use super::helper::ExifExtractor;
use crate::config::Config;
use crate::utils;
use crate::error::GalleryError;

#[derive(Debug, Template, Default)]
#[template(path = "photo.html")]
pub struct Photo {
    name: String,
    breadcrumb: Vec<(String, String)>,
    photo_full_path: PathBuf,
    album_path: String,
    previous_photo: Option<String>,
    next_photo: Option<String>,
    creation_date: String,
    flash: String,
    exposure_time: String,
    aperture: String,
    focal_length: String,
    focal_length_in_35mm: String,
    camera: String,
}

#[derive(Debug, Insertable, Queryable)]
#[table_name = "photos"]
pub struct NewPhoto {
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
    type Result = Result<NewPhoto, GalleryError>;
}

impl Message for GetPhotoId {
    type Result = Result<Option<String>, GalleryError>;
}

impl Message for GetAdjacentPhotos {
    type Result = Result<(Option<String>, Option<String>), GalleryError>;
}

impl Photo {
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

                        Ok(Photo {
                            name: photo.name,
                            breadcrumb: breadcrumb,
                            photo_full_path: PathBuf::from(""),
                            album_path: album_path,

                            previous_photo: prev,
                            next_photo: next,

                            creation_date: photo.creation_date.unwrap(),
                            flash: photo.flash.unwrap(),
                            exposure_time: photo.exposure_time.unwrap(),
                            aperture: photo.aperture.unwrap(),
                            focal_length: photo.focal_length.unwrap(),
                            focal_length_in_35mm: photo.focal_length_in_35mm.unwrap(),
                            camera: photo.camera.unwrap(),
                        })
                    },
                    Err(e) => Err(e)
                }
            })
    }

    pub fn from_path(path: PathBuf, config: &Config) -> Result<Self, GalleryError> {
        let name = path.file_name()
            .ok_or(GalleryError::InvalidFileName)?
            .to_os_string()
            .into_string()
            .map_err(|_| GalleryError::InvalidFileName)?;

        let album_path = PathBuf::from("/").join(path.parent().unwrap()).to_str().unwrap().to_string();
        let breadcrumb = utils::get_breadcrumb(&path, config);
        let photo_full_path = utils::get_album_canonical_path(path, config);

        let photo = Photo {
            name,
            breadcrumb,
            photo_full_path,
            album_path,
            ..Default::default()
        };

        photo.extract_adjacent_photos()?
            .extract_metadata()
    }

    fn extract_adjacent_photos(self) -> Result<Self, GalleryError> {
        let album_full_path = self.photo_full_path.parent().unwrap();

        let mut names: Vec<_> = fs::read_dir(album_full_path)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_file())
            .filter_map(|file| file.file_name().into_string().ok())
            .collect();

        names.sort();
        let mut iter_names = names.iter();
        let mut previous_photo = None;
        for photo_name in iter_names.by_ref() {
            if *photo_name == self.name {
                break;
            }
            previous_photo = Some(photo_name.to_string());
        }

        let next_photo = iter_names.next().map(|v| v.to_string());

        Ok(Photo {
            next_photo,
            previous_photo,
            ..self
        })
    }

    fn extract_metadata(self) -> Result<Self, GalleryError> {
        let exif_map = Self::extract_exif(&self.photo_full_path)?;

        Ok(Photo {
            creation_date: exif_map[&Tag::DateTimeOriginal].to_owned(),
            flash: exif_map[&Tag::Flash].to_owned(),
            exposure_time: exif_map[&Tag::ExposureTime].to_owned(),
            aperture: exif_map[&Tag::FNumber].to_owned(),
            focal_length: exif_map[&Tag::FocalLength].to_owned(),
            focal_length_in_35mm: exif_map[&Tag::FocalLengthIn35mmFilm].to_owned(),
            camera: utils::trim_one_char(&exif_map[&Tag::Model]),
            ..self
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
