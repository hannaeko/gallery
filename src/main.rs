use env_logger;

use actix_web::middleware::Logger;
use actix_web::{server, App, HttpRequest, http::NormalizePath, Result, Responder};

mod models;
mod gallery;
mod config;
mod utils;

fn gallery_handler(req: &HttpRequest) -> Result<impl Responder> {
    let album_path = utils::get_album_canonical_path(req.match_info().query("path")?);
    let album: models::Album = gallery::get_album_content(album_path)?;
    Ok(album)
}

fn create_app() -> App {
    App::new()
        .middleware(Logger::new("\"%r\" %Dms %s"))
        .default_resource(|r| r.h(NormalizePath::default()))
        .resource("/gallery/{path:.*}", |r| r.f(gallery_handler))
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    server::new(|| create_app())
        .bind("127.0.0.1:3000")
        .unwrap()
        .run();
}
