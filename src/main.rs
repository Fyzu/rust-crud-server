mod config;
mod database;
mod todos;

use std::io;

use actix_web::{
    error,
    middleware::{self, Logger},
    web::{self, JsonConfig},
    App, HttpResponse, HttpServer,
};
use config::get_app_config;
use database::setup_database;
use dotenvy::dotenv;
use todos::{repository::TodosRepository, service::todos_config};

#[tokio::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_config = get_app_config();

    let db = setup_database(app_config.mongodb_url, app_config.mongodb_db_name)
        .await
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    let setup_server = move || {
        let json_config = JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });

        let todos_repository = TodosRepository::new(db.clone());

        App::new()
            .app_data(json_config)
            .wrap(middleware::Compress::default())
            .wrap(Logger::default())
            .app_data(web::Data::new(todos_repository))
            .configure(todos_config)
    };

    HttpServer::new(setup_server)
        .bind((app_config.host, app_config.port))?
        .run()
        .await
}
