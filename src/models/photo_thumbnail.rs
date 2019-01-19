use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub struct PhotoThumbnail {
    name: String
}

impl PhotoThumbnail {
    pub fn new(name: String) -> Self {
        PhotoThumbnail {
            name
        }
    }

    pub fn from_path(path: PathBuf) -> io::Result<Self> {
        let name = path.file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();

        Ok(PhotoThumbnail::new(name))
    }
}
