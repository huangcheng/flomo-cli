use std::fs::File;
use std::io::prelude::*;
use std::io::{Write, Error};
use std::path::{Path, PathBuf};

use dirs::home_dir;

const FILE_NAME: &str = ".flomo";

fn get_config_file_name() -> Option<PathBuf> {
    let path = home_dir()?;
    let dir = path.to_str()?;
    Some(Path::new(dir).join(FILE_NAME))
}

pub fn save(token: &str) {
    let config = match get_config_file_name() {
        Some(file) => file,
        None => panic!("Can not get the config file path")
    };

    let mut file = match File::create(config) {
        Ok(file) => file,
        _ => panic!("Can not create the config file"),
    };

    match file.write_all(token.as_bytes()) {
        Ok(_) => println!("Token saved"),
        Err(_) => println!("Save token failed"),
    }
}

pub fn read() -> Result<String, Error> {
    let config = get_config_file_name().unwrap();
    let mut file = File::open(config)?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    Ok(content)
}