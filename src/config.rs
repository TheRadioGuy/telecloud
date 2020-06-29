use serde_derive::Deserialize;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub filesize_limit: u64,
    pub telegram_token: String,
    pub telegram_chatid: i64,
    pub server_port: u32,
}

const DEFAULT_CONFIG: &[u8] = r#"filesize_limit = 1024000 # in bytes
telegram_token = "ffff"
telegram_chatid = 1010
server_port = 1080"#
    .as_bytes();

pub fn load_config() -> Config {
    trace!("Load config");
    let path = env::current_dir().unwrap().join("config.toml");
    trace!("Config dir is {:?}", path);
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            info!("Failed to open file: {:?}, making a new one", e);
            let mut file = File::create(&path).unwrap();
            file.write_all(DEFAULT_CONFIG).unwrap();
            file.flush();
            drop(file); // FIXME: need this because we'd get `bad file descriptor` on linux on #37 line
            File::open(&path).unwrap()
        }
    };

    let config = {
        let mut string = String::new();
        file.read_to_string(&mut string).unwrap();
        string
    };

    let config: Config = toml::from_str(&config).unwrap();

    info!("Config readed: {:#?}", config);

    config
}
