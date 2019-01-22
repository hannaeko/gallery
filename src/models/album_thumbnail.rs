use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub struct AlbumThumbnail {
    pub name: String
}

impl AlbumThumbnail {
    pub fn new(name: String) -> Self {
        AlbumThumbnail {
            name
        }
    }

    pub fn from_path(path: PathBuf) -> io::Result<Self> {
        let name = path.file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();

        Ok(AlbumThumbnail::new(name))
    }
}
