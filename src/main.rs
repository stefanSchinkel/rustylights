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
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn hello_world() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::index)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello, world!");
    }
}
