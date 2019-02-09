#[macro_use]
extern crate log;
use env_logger;

use actix_web::middleware::Logger;
use actix_web::{server, App, http::NormalizePath, fs};
use actix_web::actix::System;

mod models;
mod utils;
mod routes;
mod config;
mod error;
mod common;
mod indexer;

use config::Config;
use common::AppState;

fn create_app(app_state: AppState) -> App<AppState> {
    App::with_state(app_state)
        .middleware(Logger::new("\"%r\" %Dms %s"))
        .handler("/static", fs::StaticFiles::new("./static").unwrap())
        .resource("/{path:.*}/small", |r| r.f(routes::small_thumbnail_route))
        .resource("/{path:.*}/medium", |r| r.f(routes::medium_thumbnail_route))
        .resource("/{path:.*}/full", |r| r.f(routes::full_photo_route))
        .resource("/{path:.*}", |r| r.f(routes::gallery_route))
        .default_resource(|r| r.h(NormalizePath::default()))
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=debug,gallery=debug");
    env_logger::init();
    let sys = System::new("gallery");

    let config = Config::load();
    let db_addr = models::db::init(config.db.url.clone());
    let index_addr = indexer::init(db_addr.clone());

    let app_state = AppState {
        config: config.clone(),
        db: db_addr,
        indexer: index_addr.clone()
    };

    server::new(move || create_app(app_state.clone()))
        .bind("127.0.0.1:3000")
        .unwrap()
        .start();

    index_addr.do_send(indexer::messages::IndexDirectory {
        path: std::path::PathBuf::from(config.storage_path),
        indexer: index_addr.clone()
    });

    let _ = sys.run();
}
