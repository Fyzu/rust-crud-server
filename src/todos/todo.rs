use serde::Serialize;

#[derive(Serialize)]
pub struct Todo {
    pub id: usize,
    pub text: String,
    pub is_completed: bool,
}
