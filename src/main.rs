use json::Devices;
use rocket::State;
mod json;

#[macro_use]
extern crate rocket;

fn read_cfg() -> json::Devices {
    // Parse the JSON
    let result = json::parse();

    // Handle errors from the parser if any
    match result {
        Ok(result) => {
            let devs = result.0;
            let _cfg = result.1;
            println!("++Found {} devices++", devs.devices.len());
            devs
        }
        Err(error) => {
            panic!("{}", error);
        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/bar")]
fn bar(devs: &State<Devices>) -> String {
    format!("Found {} devices", devs.devices.len())
}

#[launch]
fn rocket() -> _ {
    let devs = read_cfg();
    rocket::build()
        .manage(devs)
        .mount("/", routes![index])
        .mount("/", routes![bar])
}

// fn main() {
//     println!("Hello, world!");
// }
