use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    pub name: String,
    pub on: u32,
    pub off: u32,
}
// #[derive(Serialize, Deserialize, Debug)]
// pub struct Devices {
//     pub devices: Vec<Device>,
// }

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

#[derive(Serialize, Debug)]
pub struct Info {
    // a mini struct w/ just enough data for the FE go work
    pub index: usize,
    pub name: String,
}
impl Config {
    pub fn from_file() -> Result<Self> {
        // implemention for reading config from file
        let path = Path::new("./config.json");
        let data: String = read_to_string(path).expect("Could not read config file");
        let cfg: Config = serde_json::from_str(&data)?;
        Ok(cfg)
    }
    pub fn device_names(&self) -> Vec<Info> {
        // implemention for just name and index (all FE needs)
        self.devices
            .iter()
            .enumerate()
            .map(|(index, device)| {
                Info {
                    index,
                    // Clone the string (safe and simple)
                    name: device.name.clone(),
                }
            })
            .collect()
    }
}
