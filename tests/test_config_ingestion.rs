use std::path::PathBuf;
use lib::config;
use lib::config::Telegram;

#[test]
fn test_file_good() {
    let expected_config = config::Config {
        telegram: Telegram {
            token: "123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11".to_string(),
            current_chat_id: "123456789".to_string(),
        }
    };

    const FILEPATH: &str = "tests/config/sh-to-telegram.toml";
    let config_path = PathBuf::from(FILEPATH);
    println!("{config_path:?}");
    let config = config::Config::new(Some(config_path));

    assert_eq!(expected_config, config.unwrap());
}