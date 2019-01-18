use actix_web::{Responder, HttpRequest, HttpResponse, Error};

#[derive(Debug)]
pub struct Album {
    name: String,
    albums: Vec<AlbumThumbnail>,
    photos: Vec<PhotoThumbnail>
}

impl Album {
    pub fn new(name: String) -> Self {
        Album {
            name,
            albums: Vec::new(),
            photos: Vec::new()
        }
    }
}

impl Responder for Album {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, _req: &HttpRequest<S>) -> Result<Self::Item, Self::Error> {
        Ok(HttpResponse::Ok().content_type("text/plain").body(format!{"{:?}", self}))
    }
}

#[derive(Debug)]
pub struct Photo {
    name: String
}

#[derive(Debug)]
pub struct AlbumThumbnail {
    name: String
}

#[derive(Debug)]
pub struct PhotoThumbnail {
    name: String
}
