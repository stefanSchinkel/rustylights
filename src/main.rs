#[macro_use]
extern crate rocket;

use rocket::State;
mod json;
use json::Config;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/bar")]
fn bar(cfg: &State<Config>) -> String {
    format!("Found {} devices", cfg.devices.len())
}

#[launch]
fn rocket() -> _ {
    let cfg = match Config::from_file() {
        Ok(cfg) => cfg,
        Err(error) => panic!("couldn't load config {:?}", error),
    };
    rocket::build()
        .manage(cfg)
        .mount("/", routes![index])
        .mount("/", routes![bar])
}
