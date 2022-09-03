use std::env;

pub struct AppConfig {
    pub port: u16,
    pub host: String,
    pub mongodb_url: String,
    pub mongodb_db_name: String,
}

pub fn get_app_config() -> AppConfig {
    let port: u16 = env::var("PORT")
        .and_then(|p| Ok(p.parse::<u16>().expect("cannot parse PORT")))
        .unwrap_or(8080);
    let host = env::var("HOST").unwrap_or("127.0.0.1".to_owned());

    let mongodb_url = env::var("MONGODB_URL").expect("MONGODB_URL not presented");
    let mongodb_db_name = env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME not presented");

    AppConfig {
        port,
        host,
        mongodb_url,
        mongodb_db_name,
    }
}
