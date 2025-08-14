#[macro_use]
extern crate rocket;
use rocket::serde::{json::Json, Serialize};
use rocket::State;
// private modeules
mod json;
use json::Config;

// a struct for basic messages
#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Message {
    message: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/devices")]
fn get_devices(cfg: &State<Config>) -> Json<Message> {
    let msg = Message {
        message: format!("Found {} devices", cfg.devices.len()),
    };
    Json(msg)
}

#[get("/devices/<id>")]
fn get_device(cfg: &State<Config>, id: i8) -> Json<Message> {
    let msg = Message {
        message: format!(
            "details for device {} -> {}",
            id, cfg.devices[id as usize].name
        ),
    };
    Json(msg)
}
#[post("/devices/<id>/on")]
fn device_on(cfg: &State<Config>, id: i8) -> Json<Message> {
    let msg = Message {
        message: format!(
            "calling {}  {}",
            cfg.binaries.send, cfg.devices[id as usize].on
        ),
    };
    Json(msg)
}
#[post("/devices/<id>/off")]
fn device_off(cfg: &State<Config>, id: i8) -> Json<Message> {
    let msg = Message {
        message: format!(
            "calling {}  {}",
            cfg.binaries.send, cfg.devices[id as usize].on
        ),
    };
    Json(msg)
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
        .mount("/", routes![get_devices])
        .mount("/", routes![get_device])
        .mount("/", routes![device_on])
        .mount("/", routes![device_off])
}

#[cfg(test)]
mod tests;
