use mongodb::bson::{oid::ObjectId, serde_helpers::serialize_object_id_as_hex_string};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodoModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub text: String,
    pub is_completed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TodoResponse {
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub text: String,
    pub is_completed: bool,
}

impl From<TodoModel> for TodoResponse {
    fn from(
        TodoModel {
            id,
            text,
            is_completed,
        }: TodoModel,
    ) -> Self {
        Self {
            id,
            text,
            is_completed,
        }
    }
}

impl From<&TodoModel> for TodoResponse {
    fn from(
        TodoModel {
            id,
            text,
            is_completed,
        }: &TodoModel,
    ) -> Self {
        Self {
            id: *id,
            text: (*text).clone(),
            is_completed: *is_completed,
        }
    }
}
