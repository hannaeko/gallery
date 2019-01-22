use std::io;
use std::fs;
use std::path::PathBuf;

use super::helper::ExifExtractor;
use crate::config::Config;

use actix_web::{Responder, HttpRequest, HttpResponse, Error};
use exif::Tag;

#[derive(Debug)]
pub struct Photo {
    name: String,
    album_path: PathBuf,
    previous_photo: Option<String>,
    next_photo: Option<String>,
    creation_date: String,
    flash: String,
    exposure_time: String,
}

impl Photo {
    pub fn from_path(path: PathBuf, config: &Config) -> io::Result<Self> {
        let name = path.file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();


        let full_album_path = path.parent().unwrap();
        let mut names: Vec<_> = fs::read_dir(full_album_path)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_file())
            .filter_map(|file| file.file_name().into_string().ok())
            .collect();

        names.sort();

        let mut iter_names = names.iter();
        let mut previous_photo = None;

        for photo_name in iter_names.by_ref() {
            if *photo_name == name {
                break;
            }
            previous_photo = Some(photo_name.to_string());
        }

        let next_photo = iter_names.next().map(|v| v.to_string());
        let album_path = PathBuf::from("/").join(full_album_path.strip_prefix(config.storage_path).unwrap());

        let exif_map = Self::extract_exif(&path)?;
        Ok(Photo {
            name,
            album_path,
            next_photo,
            previous_photo,
            creation_date: exif_map[&Tag::DateTimeOriginal].to_owned(),
            flash: exif_map[&Tag::Flash].to_owned(),
            exposure_time: exif_map[&Tag::ExposureTime].to_owned(),
        })
    }
}

impl ExifExtractor for Photo {
    const TAG_LIST: &'static [Tag] = &[
        Tag::DateTimeOriginal,
        Tag::Flash,
        Tag::ExposureTime,
    ];
}

impl Responder for Photo {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, _req: &HttpRequest<S>) -> Result<Self::Item, Self::Error> {
        Ok(HttpResponse::Ok().content_type("text/plain").body(format!("{:#?}", self)))
    }
}
