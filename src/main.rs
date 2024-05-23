use rocket::State;
mod json;
use json::Config;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/bar")]
fn bar() -> String {
    let cfg = match Config::from_file() {
        Ok(cfg) => cfg,
        Err(error) => panic!("couldn't load config {:?}", error),
    };
    println!("{}", cfg.binaries.send);
    format!("{:?}", cfg)
    // format!("Found {} devices", result.devices.len())
    // format!("Found {} devices", devs.devices.len())
}

#[launch]
fn rocket() -> _ {
    // let devs = Cfg::from_file();
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![bar])
}

// fn main() {
//     println!("Hello, world!");
//     let result2 = Config::from_file();
//     println!("{:?}", result2);
// }
