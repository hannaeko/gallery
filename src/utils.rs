use std::fs;
use std::path::PathBuf;
use crate::config::Config;

pub fn get_album_canonical_path(album_path: PathBuf, config: &Config) -> PathBuf {
    let mut canonical_path = fs::canonicalize(&config.storage_path).unwrap();
    canonical_path.push(album_path);
    canonical_path
}

pub fn trim_one_char(s: &String) -> String {
    if s.len() < 2 {
        return s.to_string();
    } else {
        return s[1..(s.len() - 1)].to_string();
    }
}
