use std::fs;
use std::path::PathBuf;

use crate::models::helper::ExifExtractor;
use crate::config::Config;
use crate::utils;
use crate::error::GalleryError;

use askama::Template;
use exif::Tag;

#[derive(Debug, Template)]
#[template(path = "photo.html", print = "code")]
pub struct Photo {
    name: String,
    album_path: String,
    previous_photo: Option<String>,
    next_photo: Option<String>,
    creation_date: String,
    flash: String,
    exposure_time: String,
}

impl Photo {
    pub fn from_path(path: PathBuf, config: &Config) -> Result<Self, GalleryError> {
        let name = path.file_name()
            .ok_or(GalleryError::InvalidFileName)?
            .to_os_string()
            .into_string()
            .map_err(|_| GalleryError::InvalidFileName)?;


        let album_path = PathBuf::from("/").join(path.parent().unwrap()).to_str().unwrap().to_string();
        let full_path = utils::get_album_canonical_path(path, config);

        let mut names: Vec<_> = fs::read_dir(full_path.parent().unwrap())?
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

        let exif_map = Self::extract_exif(&full_path)?;
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
