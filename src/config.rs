use std::env;

pub struct AppConfig {
    pub port: u16,
    pub host: String,
}

pub fn get_app_config() -> AppConfig {
    let port: u16 = env::var("PORT")
        .and_then(|p| Ok(p.parse::<u16>().expect("cannot parse PORT")))
        .unwrap_or(8080);
    let host = env::var("HOST").unwrap_or("127.0.0.1".to_owned());

    AppConfig { port, host }
}
