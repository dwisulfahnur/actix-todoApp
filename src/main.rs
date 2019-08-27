#[macro_use]
extern crate diesel;

#[macro_use]
extern crate validator_derive;
extern crate validator;
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate failure;

use crate::config::config_app;

use actix_web::{middleware, App, HttpServer};
use listenfd::ListenFd;
use dotenv;
use diesel::prelude::*;
use db::get_db_pool;

mod config;
mod handlers;
mod models;
mod schema;
mod db;
mod errors;


fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let pool = get_db_pool();
    let mut server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .configure(config_app)
            .wrap(middleware::Logger::default())
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("127.0.0.1:8080").unwrap()
    };

    server.run()
}
