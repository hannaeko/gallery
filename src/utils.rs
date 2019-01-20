use std::path::PathBuf;
use crate::config::Config;

pub fn get_album_canonical_path(album_path: PathBuf, config: &Config) -> PathBuf {
    let mut canonical_path = PathBuf::from(config.storage_path);
    canonical_path.push(album_path);
    canonical_path
}

pub fn is_path_album(path: &PathBuf) -> bool {
    path.is_dir()
}

pub fn get_thumbnail_path(photo_path: &PathBuf, config: &Config) -> PathBuf {
    let mut thumbnail_path = PathBuf::from(config.cache_path);
    thumbnail_path.push(photo_path.strip_prefix(config.storage_path).unwrap());
    thumbnail_path.with_extension("small.jpeg")
}
