use std::io;
use std::fs;
use std::path::PathBuf;

use exif::Tag;
use crate::utils::get_thumbnail_path;

use image::GenericImageView;

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

    pub fn get_image(path: PathBuf, size: u32, square: bool) -> io::Result<PathBuf> {
        let thumbnail_path = get_thumbnail_path(&path);
        if !thumbnail_path.exists() {
            let img = image::open(&path)
                .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "Enable to build thumbnail"))?;
            let (width, height) = img.dimensions();

            let ratio = std::cmp::max(height, width) as f32 / std::cmp::min(height, width) as f32;
            let nsize = (ratio * size as f32) as u32;

            let mut thumbnail = img.resize(nsize, nsize, image::FilterType::Gaussian);

            if square {
                let (nwidth, nheight) = thumbnail.dimensions();
                let x = nwidth / 2 - size / 2;
                let y = nheight / 2 - size / 2;

                thumbnail = thumbnail.crop(x, y, size, size);
            }

            fs::create_dir_all(thumbnail_path.parent().unwrap())?;
            thumbnail.save(&thumbnail_path)?;
        }

        Ok(thumbnail_path)
    }
}

impl ExifExtractor for PhotoThumbnail {
    const TAG_LIST: &'static [Tag] = &[
        Tag::DateTimeOriginal
    ];
}
