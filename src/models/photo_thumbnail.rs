use std::io;
use std::path::PathBuf;

use exif::Tag;

use super::helper::ExifExtractor;

#[derive(Debug)]
pub struct PhotoThumbnail {
    name: String,
    creation_date: String,
}

impl PhotoThumbnail {
    pub fn from_path(path: PathBuf) -> io::Result<Self> {
        let name = path.file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();

        let exif_map = Self::extract_exif(path)?;

        Ok(PhotoThumbnail {
            name,
            creation_date: exif_map[&Tag::DateTimeOriginal].to_owned()
        })
    }
}

impl ExifExtractor for PhotoThumbnail {
    const TAG_LIST: &'static [Tag] = &[
        Tag::DateTimeOriginal
    ];
}
