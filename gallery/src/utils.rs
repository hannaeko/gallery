use std::fs;
use std::path::{PathBuf, Path};

use crate::config::Config;
use crate::error::GalleryError;

pub fn get_album_canonical_path(album_path: PathBuf, config: &Config) -> PathBuf {
    let mut canonical_path = fs::canonicalize(&config.storage_path).unwrap();
    canonical_path.push(album_path);
    canonical_path
}

pub fn get_file_name_string<P: AsRef<Path>>(path: P) -> Result<String, GalleryError> {
    path.as_ref().file_name()
        .ok_or(GalleryError::InvalidFileName)?
        .to_os_string()
        .into_string()
        .map_err(|_| GalleryError::InvalidFileName)
}

pub fn trim_one_char(s: String) -> String {
    if s.len() < 2 {
        return s.to_string();
    } else {
        return s[1..(s.len() - 1)].to_string();
    }
}

macro_rules! future_try {
    ($ex:expr) => {
        match $ex {
            Ok(res) => res,
            Err(e) => return Box::new(futures::future::err(e))
        }
    };
}
