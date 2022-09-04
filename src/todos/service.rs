use std::str::FromStr;

use actix_web::{get, patch, post, web, HttpRequest, HttpResponse, Responder};
use mongodb::bson::oid::ObjectId;
use serde::Deserialize;

use super::repository::TodosRepository;

use super::model::{TodoModel, TodoResponse};

#[get("/{id}")]
async fn get_todo_by_id(
    repository: web::Data<TodosRepository>,
    req: HttpRequest,
) -> impl Responder {
    let id = req
        .match_info()
        .get("id")
        .and_then(|s| ObjectId::from_str(s).ok());

    match id {
        Some(id) => {
            let todo = repository.get_by_id(id).await;

            match todo {
                Ok(Some(todo)) => HttpResponse::Ok().json(TodoResponse::from(todo)),
                Ok(None) => HttpResponse::NotFound().finish(),
                Err(e) => {
                    log::error!("{}", e);

                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        None => HttpResponse::BadRequest().finish(),
    }
}

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
struct TodoAddRequest {
    text: String,
    is_completed: bool,
}

#[post("/")]
async fn add_todo(
    repository: web::Data<TodosRepository>,
    input: web::Json<TodoAddRequest>,
) -> impl Responder {
    let todo = TodoModel {
        id: ObjectId::new(),
        text: input.text.clone(),
        is_completed: input.is_completed.clone(),
    };

    let result = repository.add_todo(&todo).await;

    match result {
        Ok(()) => HttpResponse::Ok().json(TodoResponse::from(todo)),
        Err(e) => {
            log::error!("{}", e);

            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TodoUpdateRequest {
    id: ObjectId,
    text: Option<String>,
    is_completed: Option<bool>,
}

#[patch("/")]
async fn update_todo(
    repository: web::Data<TodosRepository>,
    input: web::Json<TodoUpdateRequest>,
) -> impl Responder {
    let result = repository.get_by_id(input.id).await;

    match result {
        Ok(todo) => match todo {
            Some(todo) => {
                let mut todo = todo.clone();

                if let Some(text) = input.text.clone() {
                    todo.text = text;
                }

                if let Some(is_completed) = input.is_completed {
                    todo.is_completed = is_completed;
                }

                if let Err(e) = repository.save_todo(&todo).await {
                    log::error!("{}", e);

                    HttpResponse::InternalServerError().finish()
                } else {
                    HttpResponse::Ok().json(TodoResponse::from(todo))
                }
            }
            None => HttpResponse::NotFound().finish(),
        },
        Err(e) => {
            log::error!("{}", e);

            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn todos_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todos")
            .service(get_todo_by_id)
            .service(get_todos)
            .service(add_todo)
            .service(update_todo),
    );
}
