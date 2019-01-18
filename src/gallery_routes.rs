use actix_web::{HttpRequest, Result, Either};

use crate::utils::*;
use crate::models::{Album, Photo};


pub fn gallery_route(req: &HttpRequest) -> Result<Either<Album, Photo>> {
    let path = get_album_canonical_path(req.match_info().query("path")?);
    if is_path_album(&path) {
        Ok(Either::A(Album::from_path(path)?))
    } else {
        Ok(Either::B(Photo::from_path(path)?))
    }
}
