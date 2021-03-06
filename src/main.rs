#![feature(async_closure, proc_macro_hygiene, decl_macro, option_result_contains)]
#![deny(unused_must_use)]

extern crate log;
extern crate semver;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

extern crate dotenv;

#[macro_use]
pub mod macros;

mod cache;
mod config;
mod models;
mod monitoring;
mod providers;
mod routes;
mod services;
mod utils;

#[cfg(test)]
mod json;

use crate::routes::error_catchers;
use cache::redis::create_pool;
use dotenv::dotenv;
use routes::active_routes;
use std::time::Duration;
use utils::cors::CORS;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    env_logger::init();

    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_millis(
            config::internal_client_connect_timeout(),
        ))
        .build()
        .unwrap();

    rocket::build()
        .mount("/", active_routes())
        .register("/", error_catchers())
        .manage(create_pool())
        .manage(client)
        .attach(monitoring::performance::PerformanceMonitor())
        .attach(CORS())
}
