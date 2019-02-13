use std::fs;
use std::path::PathBuf;

use exif::Tag;
use image::GenericImageView;
use actix_web::actix::Message;

use crate::config::{Config, ThumbnailConfig};
use crate::error::GalleryError;
use super::helper::ExifExtractor;

#[derive(Debug, Queryable)]
pub struct PhotoThumbnail {
    pub name: String,
    pub creation_date: Option<String>,
}

pub struct GetPhotosThumbnail {
    pub parent_album_id: String,
}

impl Message for GetPhotosThumbnail {
    type Result = Result<Vec<PhotoThumbnail>, GalleryError>;
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
            creation_date: Some(exif_map[&Tag::DateTimeOriginal].to_owned())
        })
    }

    pub fn create_image(path: PathBuf, thumbnail_size: ThumbnailSize, config: &Config) -> Result<PathBuf, GalleryError> {
        let ThumbnailConfig { size, square, .. } = *thumbnail_size.get_thumbnail_config(config);

        let thumbnail_path = Self::get_image_path(&path, thumbnail_size, &config);

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

        Ok(thumbnail_path)
    }

    pub fn get_image_path(photo_path: &PathBuf, thumbnail_size: ThumbnailSize, config: &Config) -> PathBuf {
        let ThumbnailConfig { extension, .. } = thumbnail_size.get_thumbnail_config(config);

        let mut thumbnail_path = PathBuf::from(&config.cache_path);

        thumbnail_path.push(photo_path.strip_prefix(&fs::canonicalize(&config.storage_path).unwrap()).unwrap());
        thumbnail_path.with_extension(extension)
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
