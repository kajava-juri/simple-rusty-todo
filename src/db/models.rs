use serde::{Deserialize, Serialize};
use mongodb::{bson::doc, sync::Collection};

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoModel {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Counter {
    pub _id: String,
    pub seq: i64,
}

pub fn get_next_id(collection: &Collection<Counter>, counter_name: &str) -> Result<i64, mongodb::error::Error> {
    //
    let filter = doc! { "_id": counter_name };
    let update = doc! { "$inc": { "seq": 1 } };
    
    let counter = collection.find_one_and_update(filter, update).upsert(true).return_document(mongodb::options::ReturnDocument::After).run()?;
    
    match counter {
        Some(counter) => Ok(counter.seq),
        None => {
            collection.insert_one(Counter{ _id: counter_name.to_owned(), seq: 1 }).run()?;
            Ok(1)
        }
    }
}