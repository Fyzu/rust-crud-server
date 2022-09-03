use actix_web::{get, post, web, HttpResponse, Responder};
use mongodb::bson::oid::ObjectId;
use serde::Deserialize;

use super::repository::TodosRepository;

use super::model::{TodoModel, TodoResponse};

#[get("/")]
async fn get_todos(repository: web::Data<TodosRepository>) -> impl Responder {
    let todos = repository.get_all().await;

    match todos {
        Ok(todos) => HttpResponse::Ok().json(
            todos
                .iter()
                .map(TodoResponse::from)
                .collect::<Vec<TodoResponse>>(),
        ),
        Err(e) => {
            log::error!("{}", e);

            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TodoCreateRequest {
    text: String,
    is_completed: bool,
}

#[post("/")]
async fn post_todo(
    repository: web::Data<TodosRepository>,
    info: web::Json<TodoCreateRequest>,
) -> impl Responder {
    let todo = TodoModel {
        id: ObjectId::new(),
        text: info.text.clone(),
        is_completed: info.is_completed.clone(),
    };

    let result = repository.save_todo(&todo).await;

    match result {
        Ok(()) => HttpResponse::Ok().json(TodoResponse::from(todo)),
        Err(e) => {
            log::error!("{}", e);

            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn todos_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/todos").service(get_todos).service(post_todo));
}
