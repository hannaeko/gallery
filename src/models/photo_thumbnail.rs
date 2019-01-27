use std::fs;
use std::path::PathBuf;

use exif::Tag;
use crate::utils::get_thumbnail_path;
use crate::config::{Config, ThumbnailConfig};
use crate::error::GalleryError;

use image::GenericImageView;

use super::helper::ExifExtractor;

#[derive(Debug)]
pub struct PhotoThumbnail {
    pub name: String,
    pub creation_date: String,
}

impl PhotoThumbnail {
    pub fn from_path(path: PathBuf) -> Result<Self, GalleryError> {
        let name = path.file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();

        let exif_map = Self::extract_exif(&path)?;

        Ok(PhotoThumbnail {
            name,
            creation_date: exif_map[&Tag::DateTimeOriginal].to_owned()
        })
    }

    pub fn get_image(path: PathBuf, thumbnail_size: ThumbnailSize, config: &Config) -> Result<PathBuf, GalleryError> {
        let thumbnail_config = thumbnail_size.get_thumbnail_config(config);
        let size = thumbnail_config.size;
        let square = thumbnail_config.square;
        let extension = &thumbnail_config.extension;

        let thumbnail_path = get_thumbnail_path(&path, extension, &config);

        if !thumbnail_path.exists() {
            let img = image::open(&path)?;
            let (width, height) = img.dimensions();

            let thumbnail = if width < size && height < size {
                img
            } else if square {
                img.resize_to_fill(size, size, image::FilterType::Gaussian)
            } else {
                img.resize(size, size, image::FilterType::Gaussian)
            };

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


pub enum ThumbnailSize {
    Small,
    Medium,
}

impl ThumbnailSize {
    pub fn get_thumbnail_config<'a>(&self, config: &'a Config) -> &'a ThumbnailConfig {
        match self {
            ThumbnailSize::Small => &config.small_thumbnail,
            ThumbnailSize::Medium => &config.medium_thumbnail,
        }
    }
}
