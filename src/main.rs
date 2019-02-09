use env_logger;

use actix_web::middleware::Logger;
use actix_web::{server, App, http::NormalizePath, fs};

mod models;
mod utils;
mod routes;
mod config;
mod error;
mod common;

use config::Config;
use common::AppState;

fn create_app() -> App<AppState> {
    let config = Config::load();
    let db = models::db::init(config.db.url.to_owned());
    App::with_state(AppState { config, db })
        .middleware(Logger::new("\"%r\" %Dms %s"))
        .handler("/static", fs::StaticFiles::new("./static").unwrap())
        .resource("/{path:.*}/small", |r| r.f(routes::small_thumbnail_route))
        .resource("/{path:.*}/medium", |r| r.f(routes::medium_thumbnail_route))
        .resource("/{path:.*}/full", |r| r.f(routes::full_photo_route))
        .resource("/{path:.*}", |r| r.f(routes::gallery_route))
        .default_resource(|r| r.h(NormalizePath::default()))
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    server::new(|| create_app())
        .bind("127.0.0.1:3000")
        .unwrap()
        .run();
}
