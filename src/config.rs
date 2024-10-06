use std::env;
use std::sync::OnceLock;

use crate::error::{Error, Result};

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        Config::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - Ошибка загрузки конфигурации - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct Config {
    pub PAYLOAD_PATH: String,
    pub API_URL: String,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Self {
            PAYLOAD_PATH: get_env("PAYLOAD_PATH")?,
            API_URL: get_env("API_URL")?,
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}
