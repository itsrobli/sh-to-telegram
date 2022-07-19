use dirs;
use std::fs;
use serde::Deserialize;
use toml::de::Error;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Config {
    pub telegram: Telegram,
}

#[derive(Deserialize)]
pub struct Telegram {
    pub(crate) token: String,
    pub(crate) current_chat_id: String,
}


pub fn get_configs() -> Result<Config, Error> {
    let mut config_path = dirs::home_dir().unwrap();
    config_path.push("Dropbox");
    config_path.push("bridge_to_overlord");
    config_path.push("configs");
    config_path.push("sh-to-telegram.toml");
    println!("{}", dirs::home_dir().unwrap().display());
    println!("{}", config_path.display());

    let contents = fs::read_to_string(&config_path)
        .expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);

    let package_info: Config = toml::from_str(&*contents)?;

    println!("Token: {}", package_info.telegram.token);


    Ok(package_info)
}

