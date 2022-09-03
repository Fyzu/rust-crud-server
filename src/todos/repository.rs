use mongodb::{error, Collection, Database};
use tokio_stream::StreamExt;

use super::model::TodoModel;

#[derive(Clone)]
pub struct TodosRepository {
    collection: Collection<TodoModel>,
}

impl TodosRepository {
    pub fn new(db: Database) -> Self {
        let collection = db.collection::<TodoModel>("todos");

        TodosRepository { collection }
    }

    pub async fn add_todo(self: &Self, todo: &TodoModel) -> error::Result<()> {
        self.collection.insert_one(todo, None).await?;

        Ok(())
    }

    pub async fn get_all(self: &Self) -> error::Result<Vec<TodoModel>> {
        let mut cursor = self.collection.find(None, None).await?;

        let mut values: Vec<TodoModel> = Vec::new();

        //let item = cursor.next().await;
        while let Some(result) = cursor.next().await {
            match result {
                Ok(todo) => values.push(todo),
                Err(e) => log::error!("{:?}", e),
            }
        }

        Ok(values)
    }
}
