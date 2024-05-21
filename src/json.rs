use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    pub name: String,
    on: u32,
    off: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Devices {
    pub devices: Vec<Device>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Binaries {
    pub send: String,
    pub codesend: String,
    pub sniff: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub binaries: Binaries,
    pub devices: Vec<Device>,
}
//
// pub fn parse() -> Result<Config> {
//     // read data file
//     let data: String =
//         read_to_string(Path::new("./config.json")).expect("Could not read config file");
//
//     // parse JSON data to matching structs
//     // let devs: Devices = serde_json::from_str(&data)?;
//     let cfg: Config = serde_json::from_str(&data)?;
//
//     // return as tuple, cause, well C
//     Ok(cfg)
// }

// implement
impl Config {
    pub fn from_file() -> Result<Self> {
        let path = Path::new("./config.json");
        let data: String = read_to_string(path).expect("Could not read config file");
        let cfg: Config = serde_json::from_str(&data)?;
        Ok(cfg)
    }
}
