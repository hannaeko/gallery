use env_logger;

use actix_web::middleware::Logger;
use actix_web::{server, App, http::NormalizePath};

mod models;
mod utils;
mod routes;
mod config;

use config::Config;

fn create_app() -> App<Config> {
    App::with_state(Config::load())
        .middleware(Logger::new("\"%r\" %Dms %s"))
        .resource("/{path:.*}/small", |r| r.f(routes::small_thumbnail_route))
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
