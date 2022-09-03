pub mod config;
mod todos;

use actix_web::{error, middleware, web::JsonConfig, App, HttpResponse, HttpServer};
use config::get_app_config;
use dotenvy::dotenv;
use todos::service::todos_config;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let app_config = get_app_config();

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
