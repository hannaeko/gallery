use std::io;
use std::path::PathBuf;

use super::helper::ExifExtractor;

use actix_web::{Responder, HttpRequest, HttpResponse, Error};
use exif::Tag;

#[derive(Debug)]
pub struct Photo {
    name: String,
    creation_date: String,
    flash: String,
    exposure_time: String,
}

impl Photo {
    pub fn from_path(path: PathBuf) -> io::Result<Self> {
        let name = path.file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();

        let exif_map = Self::extract_exif(path)?;

        Ok(Photo {
            name,
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
