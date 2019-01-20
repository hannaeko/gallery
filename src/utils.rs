use std::path::PathBuf;
use crate::config;

pub fn get_album_canonical_path(album_path: PathBuf) -> PathBuf {
    let mut canonical_path = PathBuf::from(config::GALLERY_PATH);
    canonical_path.push(album_path);
    canonical_path
}

pub fn is_path_album(path: &PathBuf) -> bool {
    path.is_dir()
}

pub fn get_thumbnail_path(photo_path: &PathBuf) -> PathBuf {
    let mut thumbnail_path = PathBuf::from(config::GALLERY_CACHE);
    thumbnail_path.push(photo_path.strip_prefix(config::GALLERY_PATH).unwrap());
    thumbnail_path.with_extension("small.jpeg")
}
