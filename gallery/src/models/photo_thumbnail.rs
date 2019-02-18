use std::fs;
use std::path::PathBuf;

use image::GenericImageView;
use actix_web::actix::Message;

use crate::config::{Config, ThumbnailConfig};
use crate::error::GalleryError;

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
    pub fn create_image(path: &PathBuf, hash: &String, thumbnail_config: &ThumbnailConfig, config: &Config) -> Result<PathBuf, GalleryError> {
        let thumbnail_path = Self::get_image_path(&hash, thumbnail_config, &config);

        let ThumbnailConfig { size, square, .. } = *thumbnail_config;

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

    pub fn get_image_path(hash: &String, thumbnail_config: &ThumbnailConfig, config: &Config) -> PathBuf {
        let extension = &thumbnail_config.extension;

        let mut thumbnail_path = PathBuf::from(&config.cache_path);
        thumbnail_path.push(hash);
        thumbnail_path.with_extension(extension)
    }
}
