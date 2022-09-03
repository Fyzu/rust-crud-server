mod todos;

use std::env;

use actix_web::{error, middleware, web::JsonConfig, App, HttpResponse, HttpServer};
use dotenvy::dotenv;
use todos::controller::todos_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port: u16 = env::var("PORT")
        .and_then(|p| Ok(p.parse::<u16>().expect("cannot parse PORT")))
        .unwrap_or(8080);
    let host = env::var("HOST").unwrap_or("127.0.0.1".to_owned());

    let factory = || {
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

    let server = HttpServer::new(factory).bind((host, port));

    server?.run().await
}
