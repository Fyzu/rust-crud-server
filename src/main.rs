mod config;
mod database;
mod todos;

use std::io;

use actix_web::{error, middleware, web::JsonConfig, App, HttpResponse, HttpServer};
use config::get_app_config;
use database::setup_database;
use dotenvy::dotenv;
use todos::service::todos_config;

#[tokio::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let app_config = get_app_config();

    let _database = setup_database(app_config.mongodb_url, app_config.mongodb_db_name)
        .await
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    let setup_server = || {
        let json_config = JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });

        App::new()
            .app_data(json_config)
            .wrap(middleware::Compress::default())
            .configure(todos_config)
    };

    HttpServer::new(setup_server)
        .bind((app_config.host, app_config.port))?
        .run()
        .await
}
