use std::collections::HashMap;
use std::env;
use std::fs::{self, OpenOptions};

use once_cell::sync::Lazy;
use parking_lot::RwLock;

use crate::consts::CONFIG_NAME;
use crate::models::config::Config;

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let path = env::current_dir().unwrap().join(CONFIG_NAME);
    let data = fs::read_to_string(path).unwrap();
    let config: Config = toml::from_str(&data).unwrap();

    config
});

pub static SAVES_OPEN_OPTIONS: Lazy<RwLock<HashMap<u32, OpenOptions>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));
