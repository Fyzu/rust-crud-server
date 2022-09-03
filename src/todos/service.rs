use actix_web::{get, put, web, Responder};
use mongodb::bson::oid::ObjectId;

use super::model::TodoResponse;

#[get("/")]
pub async fn get_todos() -> impl Responder {
    web::Json(TodoResponse {
        id: ObjectId::new(),
        text: "Some text".to_owned(),
        is_completed: false,
    })
}

#[put("/")]
pub async fn put_todos() -> impl Responder {
    web::Json(TodoResponse {
        id: ObjectId::new(),
        text: "Some text".to_owned(),
        is_completed: false,
    })
}

pub fn todos_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/todos").service(get_todos).service(put_todos));
}
