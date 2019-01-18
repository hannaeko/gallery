use env_logger;

use actix_web::middleware::Logger;
use actix_web::{server, App, http::NormalizePath};

mod models;
mod utils;
mod gallery_routes;
mod config;


fn create_app() -> App {
    App::new()
        .middleware(Logger::new("\"%r\" %Dms %s"))
        .resource("/{path:.*}", |r| r.f(gallery_routes::gallery_route))
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
