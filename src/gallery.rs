use std::path::PathBuf;
use std::io;

use crate::models::Album;
use crate::config;

pub fn get_album_content(path: PathBuf) -> io::Result<Album> {
    let name = if let Some(file_name) = path.file_name() {
        file_name.to_os_string().into_string().unwrap()
    } else {
        String::from(config::GALLERY_NAME)
    };
    Ok(Album::new(name))
}
