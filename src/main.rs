#[macro_use]
extern crate rocket;

mod api;

use std::option_env;

use rocket::fs::FileServer;

const BUILD_PATH: Option<&'static str> = option_env!("BUILD_PATH");

#[launch]
fn rocket() -> _ {
    match BUILD_PATH {
        Some(path) => rocket::build().mount("/", FileServer::from(path)),
        None       => rocket::build()
    }.mount("/api", routes![api::test_func])
}
