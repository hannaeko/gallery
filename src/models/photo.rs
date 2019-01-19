use std::io;
use std::path::PathBuf;

use actix_web::{Responder, HttpRequest, HttpResponse, Error};

#[derive(Debug)]
pub struct Photo {
    name: String
}

impl Photo {
    pub fn new(name: String) -> Self {
        Photo {
            name,
        }
    }

    pub fn from_path(path: PathBuf) -> io::Result<Self> {
        let name = path.file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();

        Ok(Photo::new(name))
    }
}

impl Responder for Photo {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, _req: &HttpRequest<S>) -> Result<Self::Item, Self::Error> {
        Ok(HttpResponse::Ok().content_type("text/plain").body(format!("{:?}", self)))
    }
}
