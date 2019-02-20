#[macro_use] extern crate log;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde;

use std::path::Path;

use env_logger;
use actix_web::middleware::Logger;
use actix_web::{server, App, http::{NormalizePath, Method}, fs};
use actix_web::actix::System;

mod models;
#[macro_use]
mod utils;
mod routes;
mod config;
mod error;
mod common;
mod indexer;
mod handlers;

use config::Config;
use common::AppState;

fn create_app(app_state: AppState) -> App<AppState> {
    let static_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("static");
    App::with_state(app_state)
        .middleware(Logger::new("\"%r\" %Dms %s"))
        .scope("/admin", |admin_scope| {
            admin_scope
                .resource("/jobs", |r| {
                    r.method(Method::GET).with_async(routes::get_jobs_route);
                    r.method(Method::POST).with_async(routes::post_jobs_route)
                })
        })
        .handler("/static", fs::StaticFiles::new(static_path).unwrap())
        .resource("/{path:.*}/{thumbnail_size:small|medium}", |r| r.with_async(routes::thumbnail_route))
        .resource("/{path:.*}/full", |r| r.f(routes::full_photo_route))
        .resource("/{path:.*}", |r| r.with_async(routes::gallery_route))
        .default_resource(|r| r.h(NormalizePath::default()))
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=debug,gallery=debug");
    env_logger::init();
    let sys = System::new("gallery");

    let config = Config::load();
    let db_addr = models::db::init(config.db.url.clone());
    let index_addr = indexer::indexer_actor::IndexerActor::init(db_addr.clone(), config.clone());
    let walker_addr = indexer::walker_actor::WalkerActor::init(db_addr.clone(), index_addr.clone(), config.clone());

    let app_state = AppState {
        config: config,
        db: db_addr,
        walker: walker_addr
    };

    server::new(move || create_app(app_state.clone()))
        .bind("127.0.0.1:3000")
        .unwrap()
        .start();

    let _ = sys.run();
}
