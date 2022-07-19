use dirs;
use std::fs;
use serde::Deserialize;
use toml::de::Error;
use std::collections::HashMap;

#[derive(Deserialize)]
struct Config {
    telegram: Telegram,
}

#[derive(Deserialize)]
struct Telegram {
    token: String,
    current_chat_id: String,
}


pub fn get_configs() -> Result<(), Error> {
    // let mut dotenv_path = dirs::home_dir().unwrap();
    // dotenv_path.push("Dropbox");
    // dotenv_path.push("bridge_to_overlord");
    // dotenv_path.push("configs");
    // dotenv_path.push("sh-to-telegram.env");
    // println!("{}", dirs::home_dir().unwrap().display());
    // println!("{}", dotenv_path.display());

    let toml_content = r#"
        [telegram]
        token = "fake token"
        current_chat_id = "fake chat ID"
    "#;

    let package_info: Config = toml::from_str(toml_content)?;

    println!("Token: {}", package_info.telegram.token);

    // let contents = fs::read_to_string(&dotenv_path)
    //     .expect("Something went wrong reading the file");
    // println!("With text:\n{}", contents);
    //
    // dotenv::from_path(dotenv_path.as_path()).expect("dot env not found");
    // dotenv::dotenv().expect("Failed to read .env file");
    Ok(())
}

