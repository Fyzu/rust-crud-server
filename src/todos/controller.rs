use actix_web::{get, put, web, Responder};

use crate::todos::todo::Todo;

#[get("/")]
pub async fn get_todos() -> impl Responder {
    web::Json(Todo {
        id: 0,
        text: "Some text".to_owned(),
        is_completed: false,
    })
}

#[put("/")]
pub async fn put_todos() -> impl Responder {
    web::Json(Todo {
        id: 0,
        text: "Some text".to_owned(),
        is_completed: false,
    })
}

pub fn todos_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todos")
            .service(get_todos)
            .service(put_todos)
    );
}
