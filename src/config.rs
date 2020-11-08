use serde::Deserialize;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub input: Input,
    pub output: Output
}

#[derive(Deserialize, Debug)]
pub struct Input {
    pub links: Vec<String>
}

#[derive(Deserialize, Debug)]
pub struct Output {
    pub path: String
}

impl Config {
    pub fn load_from_file(path: &str) -> Result<Config, String> {
        match File::open(path) {
            Ok(file) => {
                let mut buffered = BufReader::new(file);
                let mut str = String::new();
                buffered.read_to_string(&mut str);
                return Config::load_from_str(&str);
            },
            Err(e) => {
                return Err(String::from("could not load the file ") + path)
            },
        };
    }

    pub fn load_from_str(lines: &str) -> Result<Config, String> {
        match toml::from_str(lines) {
            Ok(config) => return Ok(config),
            Err(e) => {
                let mut err_msg = String::from("could not deserialize config: ");
                err_msg.push_str(&e.to_string());
                return Err(err_msg)
            }
        }
    }
}