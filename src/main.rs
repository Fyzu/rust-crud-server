mod config;
mod database;
mod todos;

use std::io;

use actix_web::{error, middleware, web::JsonConfig, App, HttpResponse, HttpServer};
use config::get_app_config;
use database::setup_database;
use dotenvy::dotenv;
use mongodb::bson::oid::ObjectId;
use todos::{repository::TodosRepository, service::todos_config};

use crate::todos::model::{TodoModel, TodoResponse};

#[tokio::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_config = get_app_config();

    let db = setup_database(app_config.mongodb_url, app_config.mongodb_db_name)
        .await
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    let _todos_collection = TodosRepository::new(db);

    // let res = todos_collection.add_todo(&TodoModel {
    //     id: ObjectId::new(),
    //     text: "todo 1".to_owned(),
    //     is_completed: false,
    // }).await;

    // println!("{:?}", res);
    // let todos = todos_collection.get_all().await;
    // println!("{:?}", todos);

    // let todos = todos_collection.get_all().await.unwrap();

    // let responses: Vec<TodoResponse> = todos.iter().map(TodoResponse::from).collect();
    // println!("{:?}", responses);

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
            .wrap(Logger::default())
            .configure(todos_config)
    };

    HttpServer::new(setup_server)
        .bind((app_config.host, app_config.port))?
        .run()
        .await
}
