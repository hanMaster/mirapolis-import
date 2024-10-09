use std::sync::OnceLock;

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();
    INSTANCE.get_or_init(|| Config::new())
}

#[allow(non_snake_case)]
pub struct Config {
    pub BASE_URL: String,
    pub BASE_URL_DEMO: String,
    pub BASE_URL_LOCAL: String,
    pub CREDS: String,
}

impl Config {
    fn new() -> Self {
        Self {
            BASE_URL: String::from("base_url"),
            BASE_URL_DEMO: String::from("demo_url"),
            BASE_URL_LOCAL: String::from("local_url"),
            CREDS: String::from("creds"),
        }
    }
}
