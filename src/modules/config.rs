pub struct Config {
    pub app_id: String,
    pub secret_key: String,
    pub base_url: String,
    pub base_url_demo: String,
    pub base_url_local: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            base_url: String::from(""),
            base_url_demo: String::from(""),
            base_url_local: String::from(""),
            app_id: String::from(""),
            secret_key: String::from(""),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
