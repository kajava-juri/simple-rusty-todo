use mongodb::sync::{Client, Collection, Cursor};
pub mod models;
use models::*;

pub struct Database {
    pub client: Client,
}

impl Database {
    pub fn init() -> Result<Self, mongodb::error::Error> {
        let client: Client = Client::with_uri_str("mongodb://localhost:27017")?;
        Ok(Database { client })
    }

    pub fn add_todo(&self, todo: Todo_model) -> Result<(), mongodb::error::Error> {
        let my_coll: Collection<Todo_model> = self.client
            .database("rusty_todo")
            .collection("todos");

        let _: mongodb::results::InsertOneResult = my_coll.insert_one(todo).run()?;

        Ok(())
    }

    pub fn list_todos(&self) -> Result<(), mongodb::error::Error> {
        let todo_collection: Collection<Todo_model> = self.client
            .database("rusty_todo")
            .collection::<Todo_model>("todos");

        let mut cursor: Cursor<Todo_model> = todo_collection.find(mongodb::bson::doc! {}).run()?;
        while let Some(result) = cursor.next() {
            match result {
                Ok(todo) => println!("{:?}", todo),
                Err(e) => return Err(e.into()),
            }
        }
        Ok(())
    }

}