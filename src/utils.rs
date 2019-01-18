use std::path::PathBuf;

use crate::config;

pub fn get_album_canonical_path(album_path: PathBuf) -> PathBuf {
    let mut canonical_path = PathBuf::from(config::GALLERY_PATH);
    canonical_path.push(album_path);
    canonical_path
}
