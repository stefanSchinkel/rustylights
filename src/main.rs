use std::process::Command;

#[macro_use]
extern crate rocket;
use rocket::http::Method;
use rocket::serde::{json::Json, Serialize};
use rocket::State;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions}; // Import Cors stuff
                                                                //

// private modeules
mod json;
use crate::json::{Config, Info};

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

#[get("/devlist")]
fn get_device_list(cfg: &State<Config>) -> Json<Vec<Info>> {
    Json(cfg.device_names())
}

#[get("/devices")]
fn get_devices(cfg: &State<Config>) -> Json<Message> {
    let msg = Message {
        message: format!(
            "Found {} devices. Those are: {} and {}",
            cfg.devices.len(),
            cfg.devices[0].name,
            cfg.devices[1].name
        ),
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
            cfg.binaries.codesend, cfg.devices[id as usize].on
        ),
    };
    let id = format!("{}", cfg.devices[id as usize].on);
    let cmd = format!("{}", cfg.binaries.codesend);
    let out = Command::new(cmd).arg(id).output().expect("sth broke");
    println!("call returned: {}", out.status);
    Json(msg)
}

#[post("/devices/<id>/off")]
fn device_off(cfg: &State<Config>, id: i8) -> Json<Message> {
    let msg = Message {
        message: format!(
            "calling {}  {}",
            cfg.binaries.send, cfg.devices[id as usize].off
        ),
    };
    let id = format!("{}", cfg.devices[id as usize].off);
    let cmd = format!("{}", cfg.binaries.codesend);
    let out = Command::new(cmd).arg(id).output().expect("sth broke");
    println!("call returned: {}", out.status);
    Json(msg)
}

#[launch]
fn rocket() -> _ {
    let cfg = match Config::from_file() {
        Ok(cfg) => cfg,
        Err(error) => panic!("couldn't load config {:?}", error),
    };
    // default cors settings
    // let cors = rocket_cors::CorsOptions::default().to_cors();
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all()) // Allow all origins
        .allowed_methods(
            vec![Method::Get, Method::Post] // Allow common methods
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allowed_headers(AllowedHeaders::all()) // Allow all headers
        .allow_credentials(false) // Allow cookies/auth (requires specific origin, not '*') - Be careful!
        .to_cors()
        .expect("Failed to create CORS fairing."); // Use expect for simplicity here

    rocket::build()
        .manage(cfg)
        .mount("/", routes![index])
        .mount("/", routes![get_device_list])
        .mount("/", routes![get_devices])
        .mount("/", routes![get_device])
        .mount("/", routes![device_on])
        .mount("/", routes![device_off])
        .attach(cors)
}

#[cfg(test)]
mod tests;
