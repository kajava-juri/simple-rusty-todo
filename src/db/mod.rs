use mongodb::sync::{Client, Collection, Cursor};
pub mod models;
use models::{TodoModel, Counter, get_next_id};

pub struct Database {
    pub client: Client,
}

impl Database {
    pub fn init() -> Result<Self, mongodb::error::Error> {
        let client: Client = Client::with_uri_str("mongodb://localhost:27017").unwrap_or_else(|err| {
            eprintln!("Failed to initialize database client: {}", err);
            std::process::exit(1);
        });
        Ok(Database { client })
    }

    pub fn add_todo(&self, mut todo: TodoModel) -> Result<TodoModel, mongodb::error::Error> {
        let counter_collection: Collection<Counter> = self.client
            .database("rusty_todo")
            .collection("counters");
        todo.id = get_next_id(&counter_collection, "todo_id")?;

        let my_coll: Collection<TodoModel> = self.client
            .database("rusty_todo")
            .collection("todos");

        let _: mongodb::results::InsertOneResult = my_coll.insert_one(&todo).run()?;

        Ok(todo)
    }

    pub fn list_todos(&self) -> Result<(), mongodb::error::Error> {
        let todo_collection: Collection<TodoModel> = self.client
            .database("rusty_todo")
            .collection::<TodoModel>("todos");

        let mut cursor: Cursor<TodoModel> = todo_collection.find(mongodb::bson::doc! {}).run()?;
        while let Some(result) = cursor.next() {
            match result {
                Ok(todo) => println!("{:?}", todo),
                Err(e) => return Err(e.into()),
            }
        }
        Ok(())
    }

    pub fn remove_todo(&self, id: i64) -> Result<(), mongodb::error::Error> {
        let todo_collection: Collection<TodoModel> = self.client
            .database("rusty_todo")
            .collection::<TodoModel>("todos");

        let _: mongodb::results::DeleteResult = todo_collection.delete_one(mongodb::bson::doc! { "id": id }).run()?;

        Ok(())
    }

}

